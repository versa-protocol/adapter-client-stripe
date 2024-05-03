pub fn validate() {
    std::env::var("CLIENT_ID").expect("Missing environment variable CLIENT_ID");
    std::env::var("CLIENT_SECRET").expect("Missing environment variable CLIENT_SECRET");
    std::env::var("WEBHOOK_SECRET").expect("Missing environment variable WEBHOOK_SECRET");
    std::env::var("REGISTRY_URL").expect("Missing environment variable REGISTRY_URL");
}
