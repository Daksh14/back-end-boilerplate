use super::context::Context;

use db_client::models::WaterIntake;
use juniper::{graphql_object, FieldResult};

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn waterIntake(context: &'a Context) -> FieldResult<i32> {
        Ok(WaterIntake::get(&context.0).await?)
    }
}
