# Adapter Client Stripe Tests

This directory contains unit and integration tests for the adapter-client-stripe service.

## Running Tests

### Using cargo test
```bash
cargo test
```

### Using cargo nextest (recommended)
```bash
# Install nextest if not already installed
cargo install cargo-nextest --locked

# Run all tests
cargo nextest run

# Run tests with output
cargo nextest run --no-capture

# Run specific test
cargo nextest run test_transform_interval
```

## Test Structure

### Unit Tests (`src/tests/`)
- **config_tests.rs**: Tests for environment variable validation
- **data_adapter_tests.rs**: Tests for Stripe invoice to Versa receipt transformation
- **healthz_tests.rs**: Tests for health check endpoint
- **middleware_tests.rs**: Tests for request logging middleware
- **webhook_tests.rs**: Tests for webhook handling logic

### Integration Tests (`tests/`)
- **integration_test.rs**: End-to-end tests for HTTP endpoints

## Test Coverage

The tests cover:
1. **Configuration validation** - Ensures required environment variables are present
2. **Data transformation** - Verifies correct mapping from Stripe data to Versa schema
3. **Health endpoint** - Validates service info response format
4. **Webhook handling** - Tests error cases and request validation
5. **Middleware** - Basic functionality tests

## Environment Variables

Some tests manipulate environment variables. They save and restore original values to avoid interference between tests.

Required environment variables for running the service:
- `CLIENT_ID`
- `CLIENT_SECRET`
- `WEBHOOK_SECRET`

## Notes

- Tests use the async runtime from tokio for async test functions
- The Stripe types are external dependencies, so some tests focus on the transformation logic rather than complex type construction
- Integration tests use the axum test utilities to simulate HTTP requests