use axum::{
    extract::TypedHeader,
    headers::authorization::{Authorization, Bearer},
    http::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

// #[axum::debug_handler]
pub async fn auth<B>(
    // run the `TypedHeader` extractor
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    // you can also add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    if token_is_valid(auth.token()) {
        let response = next.run(request).await;
        Ok(response)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

fn token_is_valid(token: &str) -> bool {
    // ...
    true
}

// fn set_cookie(session_token: &str) -> impl IntoResponse {
//     http::Response::builder()
//         .status(http::StatusCode::SEE_OTHER)
//         .header("Location", "/")
//         .header(
//             "Set-Cookie",
//             format!("session_token={}; Max-Age=999999", session_token),
//         )
//         .body(http_body::Empty::new())
//         .unwrap()
// }
