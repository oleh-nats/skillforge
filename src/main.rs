use axum::{routing::get, Router, response::Html};
use async_graphql::Schema;
use async_graphql_axum::GraphQL;
use std::net::SocketAddr;

mod db;
mod graphql;
mod entities;
mod services;

#[tokio::main]
async fn main() {
    let db = db::init().await.expect("DB connection failed");

    let schema = Schema::build(
        graphql::schema::QueryRoot,
        graphql::schema::MutationRoot,
        async_graphql::EmptySubscription,
    )
    .data(db)
    .finish();

    let app = Router::new()
        .route("/", get(playground))
        .route("/graphql", GraphQL::new(schema));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn playground() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="utf-8"/>
            <title>GraphQL Playground</title>
            <link rel="stylesheet" href="//cdn.jsdelivr.net/npm/graphql-playground-react@1.7.42/build/static/css/index.css" />
            <link rel="shortcut icon" href="//cdn.jsdelivr.net/npm/graphql-playground-react@1.7.42/build/favicon.png" />
            <script src="//cdn.jsdelivr.net/npm/graphql-playground-react@1.7.42/build/static/js/middleware.js"></script>
        </head>
        <body>
            <div id="root"/>
            <script>
                window.addEventListener('load', function () {
                    GraphQLPlayground.init(document.getElementById('root'), {
                        endpoint: '/graphql'
                    })
                })
            </script>
        </body>
        </html>
    "#)
}
