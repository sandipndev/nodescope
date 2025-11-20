use async_graphql::{EmptyMutation, EmptySubscription, Schema};

mod schema;
pub use schema::*;

use app::NodeScopeApp;

pub fn schema(app: Option<NodeScopeApp>) -> Schema<Query, EmptyMutation, EmptySubscription> {
    let mut schema_builder = Schema::build(Query, EmptyMutation, EmptySubscription);

    if let Some(app) = app {
        schema_builder = schema_builder.data(app);
    }

    // TODO: Use dataloader

    schema_builder.finish()
}
