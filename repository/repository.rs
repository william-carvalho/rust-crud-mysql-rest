use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use super::schema::posts;


impl Post {
    pub fn create(post: NewPost, connection: &MysqlConnection) -> Post {
        diesel::insert_into(posts::table)
            .values(&post)
            .execute(connection)
            .expect("Error creating new post");

        posts::table.order(posts::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &MysqlConnection) -> Vec<Post> {
        posts::table.order(posts::id.asc()).load::<Post>(connection).unwrap()
    }

    pub fn update(id: i32, post: IgnoreNoneFieldsUpdatePost, connection: &MysqlConnection) -> bool {
        diesel::update(posts::table.find(id)).set(&post).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(posts::table.find(id)).execute(connection).is_ok()
    }
}