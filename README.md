# rblog

Blog engine written in rust.

## Installing

```bash
cargo install rblog
```

## Running from source code

```
git clone https://github.com/prabirshrestha/rblog.git
cargo run
```

### Running from source with listenfd

```
cargo install systemfd
systemfd --no-pid -s http::8080 -- cargo watch -x 'run'
```

## NGINX configuration

Make sure to set the `proxy_http_version 1.1` if using nginx proxy.

```
location / {
    proxy_http_version 1.1;
    .....
}
```

## License

MIT
