use serde_json::Value;
use crate::query::query::*;
use std::collections::HashMap;
use crate::ScyllaClient;
use scylla::QueryResult;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

impl<'a> QueryBuilder<'a> {
    pub fn new(operation: Operations, keyspace: &str, table: &str, client: &'a ScyllaClient) -> Self {
        Self {
            operation,
            keyspace: keyspace.to_string(),
            table: table.to_string(),
            columns: Vec::new(),
            conditions: Vec::new(),
            clauses: Vec::new(),
            order: None,
            insert_options: Vec::new(),
            client,
        }
    }
    
    pub fn select(mut self, columns: &[&str]) -> Self {
        self.columns = columns.iter().map(|&col| col.to_string()).collect();
        self
    }

    pub fn delete(mut self) -> Self {
        self.operation = Operations::Delete;
        self
    }

    pub fn update(mut self, values: HashMap<&str, &str>) -> Self {
        self.operation = Operations::Update;
        self.columns = values.into_iter()
            .map(|(col, val)| format!("{} = '{}'", col, val))
            .collect();
        self
    }

    pub fn insert<'b>(mut self, json_body: Value) -> Pin<Box<dyn Future<Output = Result<QueryResult, Box<dyn Error + Send + Sync>>> + Send + 'b>>
    where 'a: 'b {
        Box::pin(async move {
            self.operation = Operations::Insert;
            let json_string = json_body.to_string();
            self.clauses.push(format!("JSON '{}'", json_string));
            let query = self.build();
            self.client.session.query(query, &[]).await.map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        })
    }
    pub async fn execute(&self, query: String) -> Result<QueryResult, Box<dyn Error + Send + Sync>> {
        self.client.session.query(query, &[]).await.map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
    }


    pub fn where_condition(mut self, condition: &str) -> Self {
        self.conditions.push(condition.to_string());
        self
    }

    pub fn eq(mut self, column: &str, value: &str) -> Self {
        let condition = format!("{} = '{}'", column, value);
        self.conditions.push(condition);
        self
    }

    pub fn neq(mut self, column: &str, value: &str) -> Self {
        let condition = format!("{} != '{}'", column, value);
        self.conditions.push(condition);
        self
    }

    pub fn gt(mut self, column: &str, value: &str) -> Self {
        let condition = format!("{} > '{}'", column, value);
        self.conditions.push(condition);
        self
    }

    pub fn gte(mut self, column: &str, value: &str) -> Self {
        let condition = format!("{} >= '{}'", column, value);
        self.conditions.push(condition);
        self
    }

    pub fn lt(mut self, column: &str, value: &str) -> Self {
        let condition = format!("{} < '{}'", column, value);
        self.conditions.push(condition);
        self
    }

    pub fn lte(mut self, column: &str, value: &str) -> Self {
        let condition = format!("{} <= '{}'", column, value);
        self.conditions.push(condition);
        self
    }

    pub fn in_list(mut self, column: &str, values: &[&str]) -> Self {
        let value_list = values.join(", ");
        let condition = format!("{} IN ({})", column, value_list);
        self.conditions.push(condition);
        self
    }

    pub fn not_in_list(mut self, column: &str, values: &[&str]) -> Self {
        let value_list = values.join(", ");
        let condition = format!("{} NOT IN ({})", column, value_list);
        self.conditions.push(condition);
        self
    }

    pub fn between(mut self, column: &str, lower: &str, upper: &str) -> Self {
        let condition = format!("{} BETWEEN '{}' AND '{}'", column, lower, upper);
        self.conditions.push(condition);
        self
    }

    pub fn not_between(mut self, column: &str, lower: &str, upper: &str) -> Self {
        let condition = format!("{} NOT BETWEEN '{}' AND '{}'", column, lower, upper);
        self.conditions.push(condition);
        self
    }

    pub fn like(mut self, column: &str, pattern: &str) -> Self {
        let condition = format!("{} LIKE '{}'", column, pattern);
        self.conditions.push(condition);
        self
    }

    pub fn is_null(mut self, column: &str) -> Self {
        let condition = format!("{} IS NULL", column);
        self.conditions.push(condition);
        self
    }

    pub fn is_not_null(mut self, column: &str) -> Self {
        let condition = format!("{} IS NOT NULL", column);
        self.conditions.push(condition);
        self
    }

    pub fn clause(mut self, clause: &str) -> Self {
        self.clauses.push(clause.to_string());
        self
    }

    pub fn order_by(mut self, column: &str, direction: OrderDirection) -> Self {
        self.order = Some((column.to_string(), direction));
        self
    }

    pub fn insert_option(mut self, option: InsertOptions) -> Self {
        self.insert_options.push(option);
        self
    }

    pub fn build(&self) -> String {
        let operation = match self.operation {
            Operations::Select => "SELECT",
            Operations::Insert => "INSERT INTO",
            Operations::InsertIfNotExists => "INSERT IF NOT EXISTS",
            Operations::Update => "UPDATE",
            Operations::Delete => "DELETE",
        };

        let columns = if self.columns.is_empty() {
            if self.operation == Operations::Delete {
                "".to_string()
            } else {
                "*".to_string()
            }
        } else {
            self.columns.join(", ")
        };

        let full_table_name = format!("{}.{}", self.keyspace, self.table);
        let mut query = match self.operation {
            Operations::Select => format!("{} {} FROM {}", operation, columns, full_table_name),
            Operations::Delete => format!("{} FROM {}", operation, full_table_name),
            Operations::Insert | Operations::InsertIfNotExists => {
                if !self.clauses.is_empty() && self.clauses[0].starts_with("JSON") {
                    format!("{} {} {}", operation, full_table_name, self.clauses.join(" "))
                } else {
                    format!("{} {}", operation, full_table_name)
                }
            },
            Operations::Update => format!("{} {}", operation, full_table_name),
        };

        if self.operation == Operations::Update && !self.columns.is_empty() {
            query.push_str(" SET ");
            query.push_str(&columns);
        }

        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.conditions.join(" AND "));
        }

        if let Some((ref col, ref dir)) = self.order {
            let dir_str = match dir {
                OrderDirection::Asc => "ASC",
                OrderDirection::Desc => "DESC",
            };
            query.push_str(&format!(" ORDER BY {} {}", col, dir_str));
        }

        if !self.clauses.is_empty() && !self.clauses[0].starts_with("JSON") {
            query.push_str(" ");
            query.push_str(&self.clauses.join(" "));
        }

        if !self.insert_options.is_empty() {
            query.push_str(" USING ");
            let options: Vec<String> = self.insert_options.iter().map(|option| {
                match option {
                    InsertOptions::UsingTimestamp(ts) => format!("TIMESTAMP {}", ts),
                    InsertOptions::UsingTTL(ttl) => format!("TTL {}", ttl),
                }
            }).collect();
            query.push_str(&options.join(" AND "));
        }

        query.push(';');
        query
    }
}