// Middleware tests
// Note: Testing axum middleware properly requires integration tests
// These are basic unit tests for the logging functions

#[test]
fn test_middleware_module_exists() {
    // Basic test to ensure the middleware module compiles
    use adapter_client_stripe::middleware::log_request;

    // The function exists and can be referenced
    let _ = log_request;
}
