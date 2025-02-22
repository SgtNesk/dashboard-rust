mod database;
mod handlers;
mod models;

use warp::Filter;
use std::sync::Arc;
use database::Database;

#[tokio::main]
async fn main() {
    let db = Arc::new(Database::new().expect("Errore inizializzazione DB"));

    let routes = warp::path("courses")
        .and(warp::get())
        .and(warp::any().map(move || db.clone()))
        .and_then(handlers::get_courses)
        .or(warp::path("add")
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map(move || db.clone()))
            .and_then(handlers::add_course));

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
