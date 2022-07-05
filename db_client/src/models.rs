use crate::context::Context;
use crate::schema::{water_intake, water_intake::dsl::*, water_intake::intake};

use anyhow::Result;
use diesel::{result::Error, ExpressionMethods, QueryDsl, RunQueryDsl};

#[derive(Insertable)]
#[table_name = "water_intake"]
pub struct WaterIntake {
    pub intake: i32,
}

impl WaterIntake {
    pub fn new(intake_value: i32) -> Self {
        Self {
            intake: intake_value,
        }
    }

    // update only one row for now
    pub async fn update(intake_value: i32, context: &Context) -> Result<bool> {
        let conn = &*context.conn().await?;

        let entry = water_intake
            .filter(id.eq(1))
            .select(intake)
            .first::<i32>(conn);

        if let Err(entry_fetch_err) = entry {
            if entry_fetch_err == Error::NotFound {
                return Ok(diesel::insert_into(water_intake)
                    .values(Self::new(intake_value))
                    .execute(conn)
                    .is_ok());
            }
        }

        Ok(diesel::update(water_intake.filter(id.eq(1)))
            .set(intake.eq(intake_value))
            .execute(conn)
            .is_ok())
    }

    // gets only one row for now
    pub async fn get(context: &Context) -> Result<i32> {
        let conn = &*context.conn().await.unwrap();

        Ok(water_intake.filter(id.eq(1)).select(intake).first(conn)?)
    }
}
