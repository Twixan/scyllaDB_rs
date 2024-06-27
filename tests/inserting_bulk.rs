
use std::error::Error;

use serde_json::json;

use scylladb_rs::ScyllaClient;
use scylladb_rs::query::utils::print_query_result;

#[tokio::test]
async fn inserting_bulk() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
    let json_body = json!([
        {
            "age": 33,
            "name": "Johnny Doe the first",
            "score": 100.0
        },
        {
            "age": 22,
            "name": "Johnny Doe the second",
            "score": 88.5
        }
        // Add more JSON objects as needed
    ]);


    let insert_bulk: Result<scylla::QueryResult, Box<dyn Error + Sync + Send>> = client
        .query("test_keyspace", "test_table")
        .insert_bulk(json_body)
        .await;

    match &insert_bulk {
        Ok(query_result) => print_query_result("Query:", query_result),
        Err(e) => println!("Query failed: {:?}", e),
    }

    Ok(())
}

