use chrono::NaiveDateTime;

use crate::schema::access_tokens;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct AccessToken {
    pub id: i64,
    pub account_id: i64,
    pub token: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "access_tokens"]
pub struct NewAccessToken<'a> {
    pub account_id: i64,
    pub token: &'a str,
}
