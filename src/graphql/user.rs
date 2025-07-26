use async_graphql::Object;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn hello(&self) -> &str {
        "Hello from UserQuery"
    }
}
