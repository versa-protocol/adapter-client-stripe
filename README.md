# adapter-client-stripe

An "adapter client" designed to consume webhook events from Stripe and forward them to the Versa protocol

## Prerequisites

Before you can send join the network, you'll need to register with the protocol.

1. Sign up for a developer account at https://app.versa.org
2. Configure your profile and learn about your receipt schema in the [Studio](https://app.versa.org/studio) or [Docs](https://docs.versa.org)
3. Issue client credentials for the sandbox environment
4. When ready, email support@versa.org and we'll verify your account and gate you into the production environment

## Installation

More soon... 

## Environment

You can view an example "env" file in the root of this repository. In production, you'll set each of these variables on the deployed Docker image 

```bash
 # Your Versa client ID 
CLIENT_ID=versa_cid_test_xxxxxxxxxxxxxx

# Your Versa client secret, which authenticates requests to the registry — note this should never be sent to a receiver!
CLIENT_SECRET=versa_csk_test_xxxxxxxxxx 

# A webhook secret issued by Stripe, which is used to verify the authenticity of incoming webhook events
WEBHOOK_SECRET=whsec_xxxxxxxxxxxxxxxxxx

# The URL of the Versa registry, where the client will register data hashes and decryption keys
REGISTRY_URL=https://registry.versa.org
```

## Testing the Docker Image Locally

```sh
docker run --env  CLIENT_ID=versa_cid_test_xxxxx --env CLIENT_SECRET=versa_csk_test_xxxxx --env WEBHOOK_SECRET=whsec_xxxxx --env REGISTRY_URL=https://registry.versa.org [DOCKER_IMAGE]

```