use async_graphql::{InputObject, SimpleObject};

#[derive(InputObject)]
pub struct NewUserInput {
    pub name: String,
    pub email: String,
}

#[derive(SimpleObject)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl From<crate::entities::user::Model> for User {
    fn from(model: crate::entities::user::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            email: model.email,
        }
    }
}