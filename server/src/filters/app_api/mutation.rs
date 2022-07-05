use super::context::Context;

use db_client::models::WaterIntake;
use juniper::{graphql_object, FieldResult};

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    // water_drank is in milileters (ml)
    async fn addWaterIntake<'a>(water_drank: i32, context: &'a Context) -> FieldResult<bool> {
        Ok(WaterIntake::update(water_drank, &context.0).await?)
    }
}
