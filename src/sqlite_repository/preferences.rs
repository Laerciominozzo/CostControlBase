/*
 * Author: lminozzo
 *
 */

use crate::model::PreferencesRepository;

pub struct PreferencesSqlite;

impl PreferencesRepository for PreferencesSqlite {
    fn get(&self, key: &String, user_id: i32) -> Result<String, &str> {
        unimplemented!()
    }

    fn set(&self, key: &String, value: &String, user_id: i32) -> Result<i32, &str> {
        unimplemented!()
    }

    fn delete(&self, key: &String, user_id: i32) {
        unimplemented!()
    }
}