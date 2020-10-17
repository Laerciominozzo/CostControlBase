/*
 * Author: lminozzo
 *
 */

use crate::model::{UserRepository, User};

pub struct UserSqlite;

impl UserRepository for UserSqlite{
    fn get_by_credentials(&self, username: &String, password: &String) -> Result<User, &str> {
        unimplemented!()
    }

    fn get_by_id(&self) -> Result<User, &str> {
        unimplemented!()
    }

    fn insert_or_update(&self, user: &User) -> Result<i32, &str> {
        unimplemented!()
    }
}