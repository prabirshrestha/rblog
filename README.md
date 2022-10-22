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

Set `HOST` environment variable to `0.0.0.0`.

## License

MIT
