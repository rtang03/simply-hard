# Done

-   Global Settings (`Config` crate) & live reload (`Notify` crate)
-   Grpc echo server (`Tonic` crate), with graceful shutdown
-   Distributed tracing (`Tracing`, `Opentelemetry` crate) with jaeger
-   Remote & Embedded (inmemory) database (`surreal` crate)
-   custom error (`thisError` crate)
-   docker build to alpine linux, with musl build

# Todo

-   Secure grpc
-   Badges
-   Benchmark and criterion
-   Encript / decript server contents
-   Add: chrono, url, syn, tempfile, packing_lot, rayon
-   mime, ring, tower, indicatif, slab, console
-   Key derivation function
-   Mock EchoServer

### Useful commands
```shell
# build
docker buildx build . --tag simply-hard:latest

# deploy jaeger
docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p14268:14268 jaegertracing/all-in-one:latest

# run server, when using otel feature
RUST_LOG="DEBUG" cargo run --bin simply-server

```

### jaeger
[jaeger ui](http://localhost:16686/search)