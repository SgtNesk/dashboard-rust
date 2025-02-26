mod database;
mod handlers;
mod models;

use warp::Filter;
use std::sync::Arc;
use database::Database;

#[tokio::main]
async fn main() {
    env_logger::init(); // Abilita log dettagliati
    println!("Avviando il server...");

    let db = Arc::new(Database::new().expect("Errore inizializzazione DB"));
    println!("Database inizializzato correttamente!");

    // Creiamo due cloni di `db` per evitare che venga spostato
    let db_clone1 = db.clone();
    let db_clone2 = db.clone();

    // Route per verificare se il server risponde
    let test_route = warp::path("test").map(|| {
        println!("Richiesta ricevuta su /test");
        warp::reply::html("Server OK")
    });

    let courses_route = warp::path("courses")
        .and(warp::get())
        .and(warp::any().map(move || db_clone1.clone()))
        .and_then(handlers::get_courses);

    let add_course_route = warp::path("add")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || db_clone2.clone()))
        .and_then(handlers::add_course);

    // Uniamo tutte le route
    let routes = test_route.or(courses_route).or(add_course_route);

    println!("Server avviato su http://127.0.0.1:8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
