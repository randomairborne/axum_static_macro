/// This macro takes an argument of a file path and fills out the proper statics.
/// In debug mode, it will read the file live so you can change it without recompiling the program.
/// It takes three arguments. Function name (this is what you wrap in the axum get handler), file path, and content type.
/// Does not panic in release mode. Debug mode can panic if the file does not exist.
/// 
/// ```rust
/// use axum::{
///     routing::get,
///     Router,
/// };
///
/// #[tokio::main]
/// async fn main() {
///     axum_static_macro::static_file!(static, "index.html", "text/html")
///     // build our application with a single route
///     let app = Router::new().route("/", get(static));
///     // run it with hyper on localhost:3000
///     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
///         .serve(app.into_make_service())
///         .await
///         .unwrap();
/// }
/// ```

/// static_file!(root, "index.html", "text/html")
#[macro_export]
macro_rules! static_file {
    
    ($name:ident, $path:literal, $ctype:literal) => {
        pub async fn $name() -> (StatusCode, HeaderMap, String) {
            let mut headers = http::HeaderMap::new();
            headers.insert(http::header::CONTENT_TYPE, http::HeaderValue::from_static($ctype));
            tracing::debug!("Handling static file request");
            #[cfg(not(debug_assertions))]
            let file = include_str!($path).to_string();
            #[cfg(debug_assertions)]
            let file = tokio::fs::read_to_string($path)
                .await
                .expect("Program is in debug mode and the documentation file was not found!");
            (http::StatusCode::OK, headers, file)
        }
    };
}
