use async_graphql::{Context, InputObject, Object, Result};
use sea_orm::{ActiveModelTrait, Set};
use crate::entities::user::ActiveModel;
use crate::graphql::types::User;

#[derive(InputObject)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
}

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<User> {
        let db = ctx.data::<sea_orm::DatabaseConnection>()?;

        let new_user = ActiveModel {
            name: Set(input.name),
            email: Set(input.email),
            ..Default::default()
        };

        let user = new_user.insert(db).await?;
        Ok(user.into())
    }
}
