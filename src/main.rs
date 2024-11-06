use clap::{Parser, Subcommand};
use voyageai::{
    EmbeddingModel, EmbeddingsInput, EmbeddingsRequestBuilder, InputType, RerankModel,
    RerankRequest, VoyageAiClient, VoyageConfig,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate embeddings for text
    Embed {
        /// Text to embed
        #[clap(short, long)]
        text: Vec<String>,

        /// Model to use for embeddings
        #[clap(short, long, default_value = "voyage-3")]
        model: String,
    },
    /// Rerank documents based on a query
    Rerank {
        /// Query to use for reranking
        #[clap(short, long)]
        query: String,

        /// Documents to rerank
        #[clap(short, long)]
        documents: Vec<String>,

        /// Number of top results to return
        #[clap(short, long)]
        top_k: Option<usize>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Get API key from environment
    let api_key = std::env::var("VOYAGE_API_KEY").expect("VOYAGE_API_KEY must be set");
    let config = VoyageConfig::new(api_key);
    let client = VoyageAiClient::new_with_config(config);

    handle_command(&cli, &client).await?;
    Ok(())
}

async fn handle_command(cli: &Cli, client: &VoyageAiClient) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::Embed { ref text, ref model } => {
            let model = match model.as_str() {
                "voyage-3" => EmbeddingModel::Voyage3,
                "voyage-3-lite" => EmbeddingModel::Voyage3Lite,
                "voyage-finance-2" => EmbeddingModel::VoyageFinance2,
                "voyage-multilingual-2" => EmbeddingModel::VoyageMultilingual2,
                "voyage-law-2" => EmbeddingModel::VoyageLaw2,
                _ => EmbeddingModel::Voyage3,
            };

            let request = EmbeddingsRequestBuilder::new()
                .input(EmbeddingsInput::Multiple(text.clone()))
                .model(model)
                .input_type(InputType::Document)
                .build()
                .expect("Failed to build embeddings request");
            let response = client.embeddings(request.input).await?;

            println!("Generated {} embeddings", response.data.len());
            for (i, embedding) in response.data.iter().enumerate() {
                println!("Embedding {}: {} dimensions", i, embedding.embedding.len());
            }
            Ok(())
        }

        Commands::Rerank {
            ref query,
            ref documents,
            top_k,
        } => {
            let request = RerankRequest::new(query.clone(), documents.clone(), RerankModel::Rerank2, top_k)
                .expect("Failed to create rerank request");

            let response = client
                .rerank(request)
                .await
                .expect("Failed to rerank documents");

            println!("\nReranked documents by relevance:");
            for result in response.data.iter() {
                println!(
                    "Score {:.4}: {}",
                    result.relevance_score, documents[result.index]
                );
            }
            Ok(())
        }
    }
}
