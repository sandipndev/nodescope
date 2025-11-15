use async_graphql::*;

pub struct Query;

#[Object]
impl Query {
    async fn hi(&self, _ctx: &Context<'_>) -> &str {
        "Hello, World!"
    }
}
