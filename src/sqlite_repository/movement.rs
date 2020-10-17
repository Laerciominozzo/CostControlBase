/*
 * Author: lminozzo
 *
 */

use crate::model::{MovementRepository, Movement};

pub struct MovementSqlite;

impl MovementRepository for MovementSqlite{
    fn list_all_by_user(&self, user_id: i32) -> Result<Vec<Movement>, &str> {
        unimplemented!()
    }

    fn get_by_id(&self) -> Result<Movement, &str> {
        unimplemented!()
    }

    fn insert_or_update(&self, movement: &Movement) -> Result<i32, &str> {
        unimplemented!()
    }
}