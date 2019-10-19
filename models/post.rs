use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use super::schema::posts;

#[table_name = "posts"]
#[derive(Serialize, Deserialize, Identifiable, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String
}

#[table_name = "posts"]
#[derive(Serialize, Deserialize, Insertable)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub content: &'a str
}

#[table_name = "posts"]
#[derive(Serialize, Deserialize, AsChangeset)]
pub struct IgnoreNoneFieldsUpdatePost<'a> {
    pub title: &'a str,
    pub content: &'a str
}