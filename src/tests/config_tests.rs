use std::env;

#[test]
fn test_validate_with_all_env_vars() {
    // Save current env vars
    let original_client_id = env::var("CLIENT_ID").ok();
    let original_client_secret = env::var("CLIENT_SECRET").ok();
    let original_webhook_secret = env::var("WEBHOOK_SECRET").ok();

    // Set test env vars
    env::set_var("CLIENT_ID", "test_client_id");
    env::set_var("CLIENT_SECRET", "test_client_secret");
    env::set_var("WEBHOOK_SECRET", "test_webhook_secret");

    // Should not panic
    adapter_client_stripe::config::validate();

    // Restore original env vars
    match original_client_id {
        Some(val) => env::set_var("CLIENT_ID", val),
        None => env::remove_var("CLIENT_ID"),
    }
    match original_client_secret {
        Some(val) => env::set_var("CLIENT_SECRET", val),
        None => env::remove_var("CLIENT_SECRET"),
    }
    match original_webhook_secret {
        Some(val) => env::set_var("WEBHOOK_SECRET", val),
        None => env::remove_var("WEBHOOK_SECRET"),
    }
}

#[test]
#[should_panic(expected = "Missing environment variable CLIENT_ID")]
fn test_validate_missing_client_id() {
    // Save current env vars
    let original_client_id = env::var("CLIENT_ID").ok();
    let original_client_secret = env::var("CLIENT_SECRET").ok();
    let original_webhook_secret = env::var("WEBHOOK_SECRET").ok();

    // Remove CLIENT_ID but set others
    env::remove_var("CLIENT_ID");
    env::set_var("CLIENT_SECRET", "test_client_secret");
    env::set_var("WEBHOOK_SECRET", "test_webhook_secret");

    // Should panic
    let result = std::panic::catch_unwind(|| {
        adapter_client_stripe::config::validate();
    });

    // Restore original env vars
    match original_client_id {
        Some(val) => env::set_var("CLIENT_ID", val),
        None => env::remove_var("CLIENT_ID"),
    }
    match original_client_secret {
        Some(val) => env::set_var("CLIENT_SECRET", val),
        None => env::remove_var("CLIENT_SECRET"),
    }
    match original_webhook_secret {
        Some(val) => env::set_var("WEBHOOK_SECRET", val),
        None => env::remove_var("WEBHOOK_SECRET"),
    }

    // Re-panic if caught
    if let Err(e) = result {
        std::panic::resume_unwind(e);
    }
}

#[test]
#[should_panic(expected = "Missing environment variable CLIENT_SECRET")]
fn test_validate_missing_client_secret() {
    // Save current env vars
    let original_client_id = env::var("CLIENT_ID").ok();
    let original_client_secret = env::var("CLIENT_SECRET").ok();
    let original_webhook_secret = env::var("WEBHOOK_SECRET").ok();

    // Set CLIENT_ID but remove CLIENT_SECRET
    env::set_var("CLIENT_ID", "test_client_id");
    env::remove_var("CLIENT_SECRET");
    env::set_var("WEBHOOK_SECRET", "test_webhook_secret");

    // Should panic
    let result = std::panic::catch_unwind(|| {
        adapter_client_stripe::config::validate();
    });

    // Restore original env vars
    match original_client_id {
        Some(val) => env::set_var("CLIENT_ID", val),
        None => env::remove_var("CLIENT_ID"),
    }
    match original_client_secret {
        Some(val) => env::set_var("CLIENT_SECRET", val),
        None => env::remove_var("CLIENT_SECRET"),
    }
    match original_webhook_secret {
        Some(val) => env::set_var("WEBHOOK_SECRET", val),
        None => env::remove_var("WEBHOOK_SECRET"),
    }

    // Re-panic if caught
    if let Err(e) = result {
        std::panic::resume_unwind(e);
    }
}

#[test]
#[should_panic(expected = "Missing environment variable WEBHOOK_SECRET")]
fn test_validate_missing_webhook_secret() {
    // Save current env vars
    let original_client_id = env::var("CLIENT_ID").ok();
    let original_client_secret = env::var("CLIENT_SECRET").ok();
    let original_webhook_secret = env::var("WEBHOOK_SECRET").ok();

    // Set CLIENT_ID and CLIENT_SECRET but remove WEBHOOK_SECRET
    env::set_var("CLIENT_ID", "test_client_id");
    env::set_var("CLIENT_SECRET", "test_client_secret");
    env::remove_var("WEBHOOK_SECRET");

    // Should panic
    let result = std::panic::catch_unwind(|| {
        adapter_client_stripe::config::validate();
    });

    // Restore original env vars
    match original_client_id {
        Some(val) => env::set_var("CLIENT_ID", val),
        None => env::remove_var("CLIENT_ID"),
    }
    match original_client_secret {
        Some(val) => env::set_var("CLIENT_SECRET", val),
        None => env::remove_var("CLIENT_SECRET"),
    }
    match original_webhook_secret {
        Some(val) => env::set_var("WEBHOOK_SECRET", val),
        None => env::remove_var("WEBHOOK_SECRET"),
    }

    // Re-panic if caught
    if let Err(e) = result {
        std::panic::resume_unwind(e);
    }
}
