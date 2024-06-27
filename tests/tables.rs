use scylladb_rs::ScyllaClient;

#[tokio::test]
async fn creating_table() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;
    println!("Connected to ScyllaDB");

    client.create_table(
        "test_keyspace", // keyspace
        "test_table", // table
        &["age"], // partition keys
        &[], // clustering keys
        &[("age", "int"), ("name", "text"), ("score", "double")], // columns
        None, // Sorting
        None // time to live
    ).await?;
    
    Ok(())
}


#[tokio::test]
async fn dropping_table() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let client: ScyllaClient = ScyllaClient::new(vec!["127.0.0.1"]).await?;

    client.drop_table(
        "test_keyspace",
        "test_table"
    ).await?;

    Ok(())
}

