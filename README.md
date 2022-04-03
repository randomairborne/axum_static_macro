# Axum Static Macro
```toml
axum_static_macro = "1"
```
This package has a single macro (static_file) which takes arguments for name, file path, and content type and fills out the proper statics. \
In debug mode, it will read the file live so you can change it without recompiling the program. \
It takes three arguments. Function name (this is what you wrap in the axum get handler), file path, and content type. \
Does not panic in release mode. Debug mode can panic if the file does not exist.

```rust
#[tokio::main]
async fn main() {
    // create our static file handler
    axum_static_macro::static_file!(static, "index.html", "text/html");
    // build our application with a single route
    let app = axum::Router::new().route("/", axum::routing::get(static));
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```