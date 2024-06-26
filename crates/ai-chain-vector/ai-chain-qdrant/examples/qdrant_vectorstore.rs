use std::sync::Arc;

use ai_chain::{schema::EmptyMetadata, traits::VectorStore};
use ai_chain_qdrant::Qdrant;
use qdrant_client::{
    prelude::{QdrantClient, QdrantClientConfig},
    qdrant::{CreateCollection, Distance, VectorParams, VectorsConfig},
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Qdrant prep
    let config = QdrantClientConfig::from_url("http://localhost:6334");
    let client = Arc::new(QdrantClient::new(Some(config)).unwrap());
    let collection_name = "my-collection".to_string();
    let embedding_size = 1536;

    if !client
        .has_collection(collection_name.clone())
        .await
        .unwrap()
    {
        client
            .create_collection(&CreateCollection {
                collection_name: collection_name.clone(),
                vectors_config: Some(VectorsConfig {
                    config: Some(qdrant_client::qdrant::vectors_config::Config::Params(
                        VectorParams {
                            on_disk: None,
                            size: embedding_size,
                            distance: Distance::Cosine.into(),
                            hnsw_config: None,
                            quantization_config: None,
                            datatype: None,
                        },
                    )),
                }),
                ..Default::default()
            })
            .await
            .unwrap();
    }

    let embeddings = ai_chain_openai::embeddings::Embeddings::default();

    // Storing documents
    let qdrant: Qdrant<ai_chain_openai::embeddings::Embeddings, EmptyMetadata> = Qdrant::new(
        client.clone(),
        collection_name.clone(),
        embeddings,
        None,
        None,
        None,
    );
    let doc_ids = qdrant
        .add_texts(vec![
            "This is an amazing way of writing LLM-powered applications".to_string(),
        ])
        .await
        .unwrap();

    println!("Vectors stored under IDs: {:?}", doc_ids);

    let response = client
        .get_points(
            collection_name,
            None,
            &doc_ids.into_iter().map(|id| id.into()).collect::<Vec<_>>(),
            Some(true),
            Some(true),
            None,
        )
        .await
        .unwrap();

    println!("Retrieved stored vectors: {:?}", response.result);
}
