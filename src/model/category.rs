/* 
 * Author: lminozzo
 *
 */

pub struct Category{
    id      : i32,
    account : String,
    category: String,
    sign    : String,
    icon    : String,
    number  : i32,
    selected: i32,
    user    : i32
}

pub trait CategoryRepository{
    fn list_all_user_categories(&self, user_id:i32)-> Result<Vec<Category>,&str>;
    fn list_all_no_user_categories(&self) -> Result<Vec<Category>,&str>;
    fn list_all(&self, user_id:i32) -> Result<Vec<Category>,&str>;
    fn get_by_id(&self) -> Result<Category,&str>;
    fn insert_or_update(&self, category:&Category) -> Result<i32,&str>;
}