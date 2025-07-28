use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State, response::{Html, IntoResponse}, routing::{get, post}, Router
};
use async_graphql::{http::GraphQLPlaygroundConfig, EmptySubscription, Schema};
use graphql::schema::create_schema;
use tokio::net::TcpListener;
use std::{net::SocketAddr, sync::Arc};
use async_graphql::http::playground_source;

use crate::graphql::{mutation::MutationRoot, query::QueryRoot};

mod db;
mod graphql;
mod entities;
mod services;

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

async fn graphql_handler(
    State(schema): State<Arc<Schema<QueryRoot, MutationRoot, EmptySubscription>>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to read .env file");
    let db = db::init().await.expect("DB connection failed");

    let schema = create_schema(db);
    let app = Router::new()
        .route("/", get(graphql_playground))
        .route("/graphql", post(graphql_handler))
        .with_state(schema.into());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("🚀 Running at http://{}", addr);
    axum::serve(listener, app)
        .await
        .unwrap();
}
