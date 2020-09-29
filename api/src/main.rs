#[macro_use]
extern crate diesel;

use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use listenfd::ListenFd;

mod errors;
mod handlers;
mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/posts", web::get().to(handlers::get_posts))
            .route("/posts/{id}", web::get().to(handlers::get_post_by_id))
            .route("/posts", web::post().to(handlers::add_post))
            .route("/posts/{id}", web::delete().to(handlers::delete_post))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8080")?
    };

    server.run().await
}
