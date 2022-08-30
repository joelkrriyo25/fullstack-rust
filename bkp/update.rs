#[macro_use] extern crate diesel;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DB URL vairable cant't found");
    let conn = PgConnection::establish(&db_url).expect("We can't connect to the DB");

    use self::models::{Post, NewPost, PostSimple};
    use self::schema::posts::dsl::*; 
    use self::schema::posts;

    let update = diesel::update(posts.filter(id.eq(4))).set(slug.eq("cuarto post")).get_result::<Post>(&conn).expect("Error when trying to execute the query.");

    // select * from x
    println!("Query all");
    let posts_result = posts.load::<Post>(&conn).expect("Error when trying to execute the query.");

    for post in posts_result {
        println!("{:?}", post);
    }

}