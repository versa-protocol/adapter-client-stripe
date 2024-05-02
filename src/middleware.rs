use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

fn log_inbound() {
    info!("Inbound");
}
fn log_outbound(parts: &http::response::Parts) {
    info!("Outcome {}", parts.status);
}

pub async fn log_request(
    req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    log_inbound();
    let res = next.run(req).await;

    let (res_parts, body) = res.into_parts();
    log_outbound(&res_parts);
    let res = Response::from_parts(res_parts, body);

    Ok(res)
}
