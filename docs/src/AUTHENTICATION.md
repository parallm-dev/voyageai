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
# Authentication

## API Key Setup

The VoyageAI SDK requires an API key for authentication. You can obtain one from the [VoyageAI Dashboard](https://voyage.ai).

### Setting the API Key

There are several ways to provide your API key:

1. **Environment Variable**
   ```bash
   export VOYAGE_API_KEY="your-api-key-here"
   ```
   or
   ```bash
   export VOYAGEAI_API_KEY="your-api-key-here"
   ```

2. **Direct Client Configuration**
   ```rust
   let client = VoyageBuilder::new()
       .with_api_key("your-api-key-here")
       .build()?;
   ```

3. **Configuration Object**
   ```rust
   let config = VoyageConfig::new("your-api-key-here");
   let client = VoyageAiClient::new_with_config(config);
   ```

## Best Practices

1. **Environment Variables**: Prefer using environment variables for API keys
2. **Configuration Files**: Use `.env` files for development
3. **Secrets Management**: Use proper secrets management in production
4. **Key Rotation**: Implement regular key rotation
5. **Access Control**: Use minimal required permissions

## Security Considerations

- Never commit API keys to version control
- Use environment-specific keys (development/staging/production)
- Monitor API key usage for unusual patterns
- Implement proper error handling for authentication failures
