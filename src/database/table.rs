use std::error::Error;

use scylla::Session;
use scylla::QueryResult;
use scylla::transport::query_result::RowsExpectedError;
use crate::Table;

impl<'a> Table<'a> {
    //Constructor
    pub fn new(
        table_name: String,
        keyspace_name: String,
        session: &'a Session
    ) -> Self {
        Self { table_name, keyspace_name, session }
    }



    // Create an index on the specified column
    pub async fn create_index(
        &self,
        index_name: &str,
        column_name: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!("CREATE INDEX IF NOT EXISTS {} ON {}.{} ({})", index_name,
        self.keyspace_name, self.table_name, column_name);
        self.session.query(query, ()).await?;
        Ok(())
    }

    // Drop an index
    pub async fn drop_index(
        &self,
        index_name: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!("DROP INDEX IF EXISTS {}.{}",
        self.keyspace_name, index_name);
        self.session.query(query, ()).await?;
        Ok(())
    }

    // Add a column to the specified table, WORKS
    pub async fn create_column(
        &self,
        column_name: &str,
        column_datatype: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!("ALTER TABLE {}.{} ADD {} {}",
        self.keyspace_name, self.table_name, column_name, column_datatype);
        self.session.query(query, ()).await?;
        Ok(())
    }

    // Remove a column from the specified table, WORKS
    pub async fn delete_column(
        &self,
         column_name: &str
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!("ALTER TABLE {}.{} DROP {}",
        self.keyspace_name, self.table_name, column_name);
        self.session.query(query, ()).await?;
        Ok(())
    }

    // Check for duplicates in a column of the table
    pub async fn check_duplicates(
        &self,
        column_name: &str
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let query: String = format!("SELECT {}, COUNT(*) FROM {}.{} GROUP BY {} HAVING COUNT(*) > 1",
        column_name, self.keyspace_name, self.table_name, column_name);
        let query_result: QueryResult = self.session.query(query, ()).await?;
        let rows_count: Result<usize,RowsExpectedError>  = query_result.rows_num();
        match rows_count {
            Ok(count) => Ok(count > 1),
            Err(e) => Err(Box::new(e)),
        }
    }

    // Count rows in the specified table
    pub async fn count_rows(
        &self
    ) -> Result<i64, Box<dyn Error + Send + Sync>> {
        let query: String = format!("SELECT COUNT(*) FROM {}.{}",
        self.keyspace_name, self.table_name);
        let query_result: QueryResult  = self.session.query(query, ()).await?;
        let rows_count: Result<usize,RowsExpectedError> = query_result.rows_num();

        match rows_count {
            Ok(count) => Ok(count as i64),
            Err(e) => Err(Box::new(e)),
        }
    }

    // Truncate the specified table, WOKRS
    pub async fn truncate_table(
        &self
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!("TRUNCATE TABLE {}.{}",
        self.keyspace_name, self.table_name);
        self.session.query(query, ()).await?;
        Ok(())
    }
}