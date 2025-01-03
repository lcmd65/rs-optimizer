use std::any::Any;
/// Use axum capabilities.
use axum::routing::get;

/// Use tracing crates for application-level tracing output.
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Use HashMap to deserialize a HTTP GET query into a key-value map.
/// axum extracts query parameters by using `axum::extract::Query`.
/// For the implementation, see function `get_query`.
use std::collections::HashMap;

/// Use Serde JSON to serialize/deserialize JSON, such as in a request.
/// axum creates JSON or extracts it by using `axum::extract::Json`.
/// For this demo, see functions `get_demo_json` and `put_demo_json`.
use serde_json::{json, Value};

mod auth_service;
mod profile_service;
mod task_service;
mod gateway;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::event!(tracing::Level::INFO, "main");

    let app = axum::Router::new()
        .fallback(fallback)
        .route("/", get(hello))
        .route("/resources/index.html", get(hello_html))
        .route("/demo-status", get(demo_status))
        .route("/demo-uri", get(demo_uri))
        .route("/resources/demo.html", get(get_demo_html))
        .route("/demo.png", get(get_demo_png))
        .route("/demo.json", get(get_demo_json).put(put_demo_json))
        .route(
            "/foo",
            get(get_foo)
                .put(put_foo)
                .patch(patch_foo)
                .post(post_foo)
                .delete(delete_foo),
        )
        .route("/items", get(get_items))
        .route("/items/:id", get(get_items_id))
        .route("/books", get(get_users).put(put_user))
        .route("/books/:id", get(get_users_id).delete(delete_user_id))
        .route(
            "/books/:id/form",
            get(get_users_id_form),
        );

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

////
// Demo axum handlers
//
// These handlers are used to demonstrate axum capabilities.
// Each handler is an async function that returns something that
// axum can convert into a response.
////

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}

/// axum handler for "GET /" which returns a string and causes axum to
/// immediately respond with status code `200 OK` and with the string.
pub async fn hello() -> String {
    "Hello, World!".to_string()
}

/// axum handler that responds with typical HTML coming from a file.
/// This uses the Rust macro `std::include_str` to include a UTF-8 file
/// path, relative to `main.rs`, as a `&'static str` at compile time.
async fn hello_html() -> axum::response::Html<&'static str> {
    include_str!("../resources/index.html").into()
}

/// axum handler for "GET /demo-status" which returns a HTTP status
/// code, such as OK (200), and a custom user-visible string message.
pub async fn demo_status() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "Everything is OK".to_string())
}

/// axum handler for "GET /demo-uri" which shows the request's own URI.
/// This shows how to write a handler that receives the URI.
pub async fn demo_uri(uri: axum::http::Uri) -> String {
    format!("The URI is: {:?}", uri)
}

/// axum handler for "GET /demo.html" which responds with HTML text.
/// The `Html` type sets an HTTP header content-type of `text/html`.
pub async fn get_demo_html() -> axum::response::Html<&'static str> {
    "<h1>Hello</h1>".into()
}

/// axum handler for "GET /demo.png" which responds with an image PNG.
/// This sets a header "image/png" then sends the decoded image data.
async fn get_demo_png() -> impl axum::response::IntoResponse {
    use base64::Engine;
    let png = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mPk+89QDwADvgGOSHzRgAAAAABJRU5ErkJggg==";
    (
        axum::response::AppendHeaders([(axum::http::header::CONTENT_TYPE, "image/png")]),
        base64::engine::general_purpose::STANDARD
            .decode(png)
            .unwrap(),
    )
}

////
// Demo axum JSON extractor
//
// axum has capabilities for working with JSON data.
//
// The axum extractor for JSON can also help with a request, by deserializing a
// request body into some type that implements serde::Deserialize. If the axum
// extractor is unable to parse the request body, or the request does not
// contain the Content-Type: application/json header, then the axum extractor
// will reject the request and return a 400 Bad Request response.
//
// The axum extractor for JSON can help with a response, by formatting JSON data
// then setting the response application content type.
////

/// axum handler for "GET /demo.json" which returns JSON data.
/// The `Json` type sets an HTTP header content-type `application/json`.
/// The `Json` type supports types that implement `serde::Deserialize`.
pub async fn get_demo_json() -> axum::extract::Json<Value> {
    json!({"a":"b"}).into()
}

/// axum handler for "PUT /demo.json" which uses `aumx::extract::Json`.
/// This buffers the request body then deserializes it using serde.
/// The `Json` type supports types that implement `serde::Deserialize`.
pub async fn put_demo_json(
    axum::extract::Json(data): axum::extract::Json<serde_json::Value>,
) -> String {
    format!("Put demo JSON data: {:?}", data)
}

////
// Demo axum handlers with HTTP verbs GET, PUT, PATCH, POST, DELETE.
//
// The axum route has functions for each HTTP verb.
// We use the naming convention of `get_foo`, `put_foo`, etc.
//
// These demo handlers in this section simply return a string.
// After this section, you'll see how to return other data.
////

/// axum handler for "GET /foo" which returns a string message.
/// This shows our naming convention for HTTP GET handlers.
pub async fn get_foo() -> String {
    "GET foo".to_string()
}

/// axum handler for "PUT /foo" which returns a string message.
/// This shows our naming convention for HTTP PUT handlers.
pub async fn put_foo() -> String {
    "PUT foo".to_string()
}

/// axum handler for "PATCH /foo" which returns a string message.
/// This shows our naming convention for HTTP PATCH handlers.
pub async fn patch_foo() -> String {
    "PATCH foo".to_string()
}

/// axum handler for "POST /foo" which returns a string message.
/// This shows our naming convention for HTTP POST handlers.
pub async fn post_foo() -> String {
    "POST foo".to_string()
}

/// axum handler for "DELETE /foo" which returns a string message.
/// This shows our naming convention for HTTP DELETE handlers.
pub async fn delete_foo() -> String {
    "DELETE foo".to_string()
}

////
// Demo axum handlers with extractors for query params and path params.
//
// axum can automatically extract paramters from a request,
// and then pass them to a handler, using function parameters.
////

/// axum handler for "GET /items" which uses `axum::extract::Query`.
/// This extracts query parameters and creates a key-value pair map.
pub async fn get_items(
    axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>,
) -> String {
    format!("Get items with query params: {:?}", params)
}

/// axum handler for "GET /items/:id" which uses `axum::extract::Path`.
/// This extracts a path parameter then deserializes it as needed.
pub async fn get_items_id(axum::extract::Path(id): axum::extract::Path<String>) -> String {
    format!("Get items with path id: {:?}", id)
}

/////
/// Demo books using RESTful routes and a data store.
///
/// This section uses a `Book` struct, a `DATA` variable
/// that is a lazy mutex global variable, and handlers
/// that process the routes for HTTP verbs GET, PUT, etc.
/////

/// See file book.rs, which defines the `Book` struct.

/// See file data.rs, which defines the DATA global variable.
use crate::auth_service::user::User;


/// Use Thread for spawning a thread e.g. to acquire our crate::DATA mutex lock.
use std::thread;
use crate::auth_service::data::GLOBAL_USER;

/// To access data, create a thread, spawn it, then get the lock.
/// When you're done, then join the thread with its parent thread.
#[allow(dead_code)]
async fn print_data() {
    thread::spawn(move || {
        let data = GLOBAL_USER.lock().unwrap();
        println!("data: {:?}", data);
    })
        .join()
        .unwrap()
}

/// axum handler for "GET /books" which responds with a resource page.
/// This demo uses our DATA; a production app could use a database.
/// This demo must clone the DATA in order to sort items by title.
pub async fn get_users() -> axum::response::Html<String> {
    thread::spawn(move || {
        let mut data = GLOBAL_USER.lock().unwrap();
        let mut users = data.clone();
        users
            .iter()
            .map(|user| format!("<p>{}</p>\n", &user.username))
            .collect::<String>()
    })
        .join()
        .unwrap()
        .into()
}

/// This demo shows how axum can extract JSON data into a user struct.
pub async fn put_user(
    axum::extract::Json(user): axum::extract::Json<User>,
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let mut data = GLOBAL_USER.lock().unwrap();
        data.insert(user.clone());
        format!("Put user: {}", &user.username)
    })
        .join()
        .unwrap()
        .into()
}

/// axum handler for "GET /books/:id" which responds with one resource HTML page.
/// This demo app uses our crate::DATA variable, and iterates on it to find the id.
pub async fn get_users_id(
    axum::extract::Path(user): axum::extract::Path<User>,
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let mut data = GLOBAL_USER.lock().unwrap();
        match data.get(&user) {
            Some(users) => format!("<p>{}</p>\n", &users.id),
            None => format!("<p>Book id {} not found</p>", &user.username),
        }
    })
        .join()
        .unwrap()
        .into()
}

/// axum handler for "DELETE /books/:id" which destroys a resource.
/// This demo extracts an id, then mutates the book in the DATA store.
pub async fn delete_user_id(
    axum::extract::Path(id): axum::extract::Path<u32>,
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let mut data = GLOBAL_USER.lock().unwrap();
        let users =  data.clone();
        let user = users.iter().find(|&user| user.id == id).unwrap();
        if user.id != 0 {
            data.remove(user);
            format!("Delete user id: {}", &id)
        } else {
            format!("user id not found: {}", &id)
        }
    })
        .join()
        .unwrap()
        .into()
}

/// axum handler for "GET /books/:id/form" which responds with a form.
/// This demo shows how to write a typical HTML form with input fields.
pub async fn get_users_id_form(
    axum::extract::Path(user): axum::extract::Path<User>,
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = GLOBAL_USER.lock().unwrap();
        match data.get(&user) {
            Some(users) => format!(
                concat!(
                "<form method=\"post\" action=\"/users/{}/form\">\n",
                "<input type=\"hidden\" name=\"id\" value=\"{}\">\n",
                "<p><input name=\"name\" value=\"{}\"></p>\n",
                "<input type=\"submit\" value=\"Save\">\n",
                "</form>\n"
                ),
                &users.id, &users.id, &users.username
            ),
            None => format!("<p>User id {} not found</p>", &user.id),
        }
    })
        .join()
        .unwrap()
        .into()
}

pub async fn authentication(
    axum::extract::Path(user): axum::extract::Path<String>,
) {
    thread::spawn(move || {
        let mut authen = db.session.query(user);
        if authen{
            put_user(&user);
            get("index.html")
        }

        else {
            get("404.html")
        }
    })
}