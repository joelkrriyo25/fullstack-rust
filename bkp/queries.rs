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

    // let new_post = NewPost { 
    //     title: "My second blog",
    //     body: "Testtt of body",
    //     slug: "second-post", 
    // };

    // let post: Post = diesel::insert_into(posts:: table).values(&new_post).get_result(&conn).expect("The insert failed.");

    // get top 1
    let posts_result = posts.limit(1).load::<Post>(&conn).expect("Error when trying to execute the query.");

    for post in posts_result {
        println!("{:?}", post);
    }

    // select * from x
    let posts_result = posts.load::<Post>(&conn).expect("Error when trying to execute the query.");

    for post in posts_result {
        println!("{:?}", post);
    }

    // select title, body from x - column specify
    let posts_result = posts.select((title, body)).limit(1).load::<PostSimple>(&conn).expect("Error when trying to execute the query.");

    for post in posts_result {
        println!("{:?}", post);
    }

    // select title, body from x - last post by order id
    let posts_result = posts.order(id.desc()).limit(1).load::<Post>(&conn).expect("Error when trying to execute the query.");

    for post in posts_result {
        println!("{:?}", post);
    }

    // select title, body from x - filter -  wheree
    let posts_result = posts.filter(slug.eq("second-post")).limit(1).load::<Post>(&conn).expect("Error when trying to execute the query.");

    for post in posts_result {
        println!("{:?}", post);
    }

}
