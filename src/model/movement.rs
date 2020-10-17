/* ,
* Author: lminozzo,
*,
*/

pub struct Movement{
    id      :i32 ,
    account :String,
    category:i32,
    amount  :f32,
    sign    : String,
    detail  : String,
    date    : String,
    time    : String,
    confirmed: i32,
    transfer: i32,
    date_idx: String,
    day     : String,
    week    : String,
    fortnight : String,
    month   : String,
    year    : String,
    code    : String,
    picture : String,
    iso_code: String,
    selected: i32,
    user    : i32,
}

pub trait MovementRepository{
    fn list_all_by_user(&self, user_id:i32)-> Result<Vec<Movement>,&str>;
    fn get_by_id(&self) -> Result<Movement,&str>;
    fn insert_or_update(&self, movement:&Movement) -> Result<i32,&str>;
}