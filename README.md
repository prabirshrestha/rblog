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

### Running in docker

#### Running tagged docker image

```bash
docker run -v ./posts:/data/posts -v ./blog.conf:/data/blog.yaml -p 8080:8080 prabirshrestha/rblog:v0.264.0
```

#### Running latest docker image

```bash
docker run -v ./posts:/data/posts -v ./blog.conf:/data/blog.yaml -p 8080:8080 prabirshrestha/rblog:latest
```
#### Running nightly docker image

```bash
docker run -v ./posts:/data/posts -v ./blog.conf:/data/blog.yaml -p 8080:8080 prabirshrestha/rblog:nightly
```

*For demos ignore the volume mappings.*

```bash
docker run -p 8080:8080 prabirshrestha/rblog:nightly
```

## License

MIT
