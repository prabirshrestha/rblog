"""Check a real server with local fixtures and an isolated port."""

import os
from pathlib import Path
import shutil
import signal
import socket
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.request


def main():
    binary = Path(sys.argv[1]).resolve()
    with tempfile.TemporaryDirectory(prefix="rblog-smoke-") as directory:
        directory = Path(directory)
        shutil.copytree("posts", directory / "posts")
        with socket.socket() as listener:
            listener.bind(("127.0.0.1", 0))
            port = listener.getsockname()[1]
        config = directory / "blog.yaml"
        config.write_text(
            f"title: Smoke Test\nroot: http://127.0.0.1:{port}\n"
            f"host: 127.0.0.1\nport: '{port}'\n"
            "posts_dir: ./posts\nwatch:\n  enabled: true\n  interval: 1\n",
            encoding="ascii",
        )
        env = {
            "PATH": os.defpath,
            "HOME": str(directory),
            "RUST_LOG": "info",
        }
        version = subprocess.check_output(
            [binary, "version"], env=env, cwd=directory, timeout=10, text=True
        )
        assert version.startswith("rblog "), version
        with (directory / "server.log").open("w+") as log:
            process = subprocess.Popen(
                [binary, "run", "-c", config], env=env, cwd=directory,
                stdout=log, stderr=subprocess.STDOUT,
            )
            try:
                base = f"http://127.0.0.1:{port}"

                def get(path):
                    with urllib.request.urlopen(base + path, timeout=2) as response:
                        assert response.status == 200
                        return response.read()

                def wait_for(check, description):
                    deadline = time.monotonic() + 30
                    while time.monotonic() < deadline:
                        assert process.poll() is None, "Server stopped early"
                        try:
                            if check():
                                return
                        except (OSError, urllib.error.URLError):
                            pass
                        time.sleep(0.1)
                    raise AssertionError(f"Timed out: {description}")

                wait_for(lambda: get("/healthcheck") == b"OK", "startup")
                assert b"Welcome" in get("/")
                assert b"Welcome" in get("/posts/welcome/")
                assert b"<rss" in get("/rss")
                assert b"Disallow: /healthcheck" in get("/robots.txt")
                attachment = (directory / "posts/welcome/attachment.txt").read_bytes()
                assert get("/posts/welcome/attachment.txt") == attachment
                try:
                    get("/posts/missing/")
                    raise AssertionError("Missing post returned success")
                except urllib.error.HTTPError as error:
                    assert error.code == 404

                # Wait for watcher setup before changing the fixture.
                def watcher_ready():
                    log.flush()
                    return "Watching" in (directory / "server.log").read_text()

                wait_for(watcher_ready, "watcher setup")
                post = directory / "posts/welcome/welcome.md"
                content = post.read_text()
                post.write_text(content.replace("title: Welcome", "title: Reloaded Post"))
                wait_for(lambda: b"Reloaded Post" in get("/"), "post reload")
                config.write_text(config.read_text().replace("Smoke Test", "Reloaded Blog"))
                wait_for(lambda: b"Reloaded Blog" in get("/"), "config reload")
                process.send_signal(signal.SIGTERM)
                assert process.wait(timeout=10) == 0, "Shutdown failed"
                print("Live server checks passed: CLI, HTTP, post/config reload, shutdown")
            except BaseException:
                log.flush()
                print((directory / "server.log").read_text(), file=sys.stderr)
                raise
            finally:
                if process.poll() is None:
                    process.kill()
                    process.wait(timeout=10)


if __name__ == "__main__":
    main()
