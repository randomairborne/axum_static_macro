//! This macro takes an argument of a file path and fills out the proper statics.
//! In debug mode, it will read the file live so you can change it without recompiling the program. Note: You must be in the crate root for this to work.
//! It takes three arguments. Function name (this is what you wrap in the axum get handler), file path, and content type.
//! Does not panic in release mode. Debug mode can panic if the file does not exist.
//! Includes the content_types module to simplify returning static files of certain types.
//!
//! ```rust
//! #[tokio::main]
//! async fn main() {
//!     axum_static_macro::static_file!(index, "index.html", axum_static_macro::content_types::HTML);
//!     // build our application with a single route
//!     let app = axum::Router::new().route("/", axum::routing::get(index));
//!     // run it with hyper on localhost:3000
//!     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//!         .serve(app.into_make_service())
//!         .await
//!         .unwrap();
//! }
//! ```

/// A collection of content MIME types, for use as the third argument of static_file
pub use content_types;

/// Usage: `static_file!(root, "index.html", "text/html")`

#[macro_export]
macro_rules! static_file {
    ($name:ident, $path:literal, $ctype:expr) => {
        pub async fn $name() -> (axum::http::StatusCode, axum::http::HeaderMap, String) {
            let mut headers = axum::http::HeaderMap::new();
            headers.insert(
                axum::http::header::CONTENT_TYPE,
                axum::http::HeaderValue::from_static($ctype),
            );
            #[cfg(not(debug_assertions))]
            let file = include_str!($path).to_string();
            #[cfg(debug_assertions)]
            let file = tokio::fs::read_to_string(concat!("src/", $path))
                .await
                .expect(concat!(
                    "Program is in debug mode and the ",
                    $path,
                    " file was not found!"
                ));
            (axum::http::StatusCode::OK, headers, file)
        }
    };
}

