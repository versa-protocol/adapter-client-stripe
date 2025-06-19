use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use tower::ServiceExt;

// Integration test to verify the service starts and responds correctly
#[tokio::test]
async fn test_healthz_endpoint() {
    // Create the router
    let app = Router::new().route(
        "/",
        axum::routing::get(adapter_client_stripe::healthz::service_info),
    );

    // Create a request
    let request = Request::builder().uri("/").body(Body::empty()).unwrap();

    // Send the request
    let response = app.oneshot(request).await.unwrap();

    // Check the response
    assert_eq!(response.status(), StatusCode::OK);

    // Parse the body
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

    // Check that it contains expected fields
    assert!(body_str.contains("service_name"));
    assert!(body_str.contains("adapter-client-stripe"));
    assert!(body_str.contains("service_version"));
    assert!(body_str.contains("system_time"));
}

#[test]
fn test_config_validation_in_safe_environment() {
    // Save current env vars
    let saved_vars = [
        ("CLIENT_ID", std::env::var("CLIENT_ID").ok()),
        ("CLIENT_SECRET", std::env::var("CLIENT_SECRET").ok()),
        ("WEBHOOK_SECRET", std::env::var("WEBHOOK_SECRET").ok()),
    ];

    // Set test values
    std::env::set_var("CLIENT_ID", "test_id");
    std::env::set_var("CLIENT_SECRET", "test_secret");
    std::env::set_var("WEBHOOK_SECRET", "test_webhook");

    // Should not panic
    adapter_client_stripe::config::validate();

    // Restore env vars
    for (key, value) in saved_vars {
        match value {
            Some(val) => std::env::set_var(key, val),
            None => std::env::remove_var(key),
        }
    }
}
