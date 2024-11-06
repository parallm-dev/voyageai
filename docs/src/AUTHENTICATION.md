# Authentication VoyageAI Client (Rust)

Signup with VoyageAI and get your API key

You can set the environment variable `VOYAGE_API_KEY` to your API key.
This is the recommended way to authenticate your client.
`VOYAGEAI_API_KEY` is also supported for backwards compatibility.

Alternatively you can provde the api to client builder:

```rust
const client = VoyageAiClient::builder()
    .api_key("your_api_key_here")
    .build()?;
```

## VoyageAI API Authorization

All API requests require an API key for authentication. You can obtain your API key by signing up on the [VoyageAI dashboard](https://dash.voyageai.com).

Include your API key in the `Authorization` header of your HTTP requests:

```http
GET /v1/embeddings HTTP/1.1
Host: api.voyageai.com
Authorization: Bearer YOUR_API_KEY_HERE
Content-Type: application/json
```

Replace `YOUR_API_KEY_HERE` with your actual API key.
