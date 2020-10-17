/*
 * Author: lminozzo
 *
 */

pub struct Preferences{
    id  : i32,
    key : String,
    value : String,
    user  :i32
}

pub trait PreferencesRepository{
    fn get(&self, key:&String,user_id:i32) -> Result<String,&str>;
    fn set(&self, key:&String, value:&String,user_id:i32) -> Result<i32,&str>;
    fn delete(&self, key: &String,user_id:i32);
}