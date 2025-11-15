use async_graphql::{EmptyMutation, EmptySubscription, Schema};

mod schema;
pub use schema::*;

use app::NodeScopeApp;

pub fn schema(app: Option<NodeScopeApp>) -> Schema<Query, EmptyMutation, EmptySubscription> {
    let mut schema_builder = Schema::build(Query, EmptyMutation, EmptySubscription);

    // TODO: Use dataloader

    schema_builder.finish()
}
