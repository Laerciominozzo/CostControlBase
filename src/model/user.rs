/*
 * Author: lminozzo
 *
 */

pub struct User{
    id : i32,
    username : String,
    first_name: String,
    last_name: String,
    password : String,
}

pub trait UserRepository{
    fn get_by_credentials(&self, username:&String, password:&String) -> Result<User,&str>;
    fn get_by_id(&self) -> Result<User,&str>;
    fn insert_or_update(&self, user:&User) -> Result<i32,&str>;
}