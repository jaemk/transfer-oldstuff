use schema::items;
use chrono::{DateTime, UTC};


#[derive(Insertable)]
#[table_name="items"]
pub struct NewItem<'a> {
    pub unique_key: &'a str,
    pub iv: &'a [u8],
    pub bytes: &'a [u8],
    pub confirm: &'a [u8],
    pub filename: Option<Vec<u8>>,
    pub lifespan: i64,
    pub dl_limit: i32,
}


#[derive(Queryable, Debug)]
pub struct Item {
    pub id: i32,
    pub unique_key: String,
    pub iv: Vec<u8>,
    pub bytes: Vec<u8>,
    pub confirm: Vec<u8>,
    pub filename: Option<Vec<u8>>,
    pub lifespan: Option<i64>,
    pub dl_limit: Option<i32>,
    pub accesses: i32,
    pub date_created: DateTime<UTC>,
    pub date_viewed: DateTime<UTC>,
}

