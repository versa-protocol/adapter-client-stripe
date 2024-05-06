use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

fn log_inbound(uri: &str) {
    info!("Inbound {}", uri);
}
fn log_outbound(parts: &http::response::Parts, uri: &str) {
    info!("Outcome {} {}", uri, parts.status);
}

pub async fn log_request(
    uri: Option<axum::extract::MatchedPath>,
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let uri_str = uri
        .and_then(|u| Some(u.as_str().to_owned()))
        .unwrap_or_default();
    log_inbound(&uri_str);
    let res = next.run(req).await;

    let (res_parts, body) = res.into_parts();
    log_outbound(&res_parts, &uri_str);
    let res = Response::from_parts(res_parts, body);

    Ok(res)
}
