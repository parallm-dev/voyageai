#[cfg(test)]
mod tests {
    use voyageai::{EmbeddingModel, RerankModel};
    use voyageai::models::ModelType;

    #[test]
    fn test_model_type_as_str() {
        let rerank = ModelType::Rerank(RerankModel::Rerank2);
        let embedding = ModelType::Embedding(EmbeddingModel::Voyage3);

        assert_eq!(rerank.as_str(), "rerank");
        assert_eq!(embedding.as_str(), "embedding");
    }
}
