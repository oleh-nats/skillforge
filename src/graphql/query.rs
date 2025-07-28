use async_graphql::{Context, Object, Result};
use sea_orm::{EntityTrait};
use crate::entities::user::Entity as UserEntity;
use crate::graphql::types::User;

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_user_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<User>> {
        let db = ctx.data::<sea_orm::DatabaseConnection>()?;
        let user = UserEntity::find_by_id(id).one(db).await?;
        Ok(user.map(|u| u.into()))
    }
}