use async_graphql::{ EmptySubscription, Schema};
use sea_orm::DatabaseConnection;

use crate::graphql::{mutation::MutationRoot, query::QueryRoot};

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(db: DatabaseConnection) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db)
        .finish()
}
