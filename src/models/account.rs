use chrono::NaiveDateTime;

use crate::schema::accounts;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: i64,
    pub full_name: String,
    pub phone_num: String
    pub email: String, 
    pub role: String,
    pub active: bool,
    pub registration_time: NaiveDateTime
}

#[derive(Insertable, Debug)]
#[table_name="accounts"]
pub struct NewAccount<'a> {
    pub full_name: &'a str,
    pub phone_num: &'a str,
    pub email: &'a str,
    pub role: &'a str
}