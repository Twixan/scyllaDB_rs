use scylladb_rs::ScyllaClient;
use scylladb_rs::query::query::*;
use std::collections::HashMap;
#[tokio::test]
async fn integration_test() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client = ScyllaClient::new(vec!["127.0.0.1"]).await?;
    let mut values = HashMap::new();
    values.insert("email", "new_email@example.com");
    values.insert("age", "30");
    
    let query = client.query("test", "example_table")
        .update(values)
        .eq("user_id", "12345")
        .gt("age", "20")
        .build();
    
    println!("{}", query); 
    Ok(())
}