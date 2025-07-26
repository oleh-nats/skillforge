use async_graphql::{MergedObject, Object};

use super::user::UserQuery;

#[derive(MergedObject, Default)]
pub struct QueryRoot(UserQuery);

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn ping(&self) -> &str {
        "pong"
    }
}
