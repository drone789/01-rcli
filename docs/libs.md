```
cargo add tokio --features rt --features rt-multi-thread  --features net --features fs --features macros
cargo add tracing
cargo add tracing-subscriber --features env-filter
cargo add axum --features http2 --features query --features tracing

cargo add tower-http --features compression-full --features cors --features trace --features fs
```
