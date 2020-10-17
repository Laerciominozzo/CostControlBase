/*
 * Author: lminozzo
 *
 */

use crate::model::{CategoryRepository, Category};

pub struct CategorySqlite;

impl CategoryRepository for CategorySqlite{
    fn list_all_user_categories(&self, user_id: i32) -> Result<Vec<Category>, &str> {
        unimplemented!()
    }

    fn list_all_no_user_categories(&self) -> Result<Vec<Category>, &str> {
        unimplemented!()
    }

    fn list_all(&self, user_id: i32) -> Result<Vec<Category>, &str> {
        unimplemented!()
    }

    fn get_by_id(&self) -> Result<Category, &str> {
        unimplemented!()
    }

    fn insert_or_update(&self, movement: &Category) -> Result<i32, &str> {
        unimplemented!()
    }
}