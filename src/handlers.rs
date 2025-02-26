use warp::reply::Json;
use std::sync::Arc;
use rusqlite::params;
use crate::database::Database;
use crate::models::Course;

pub async fn get_courses(db: Arc<Database>) -> Result<Json, warp::Rejection> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, name, hours_attended FROM courses").unwrap();
    let courses = stmt
        .query_map([], |row| {
            Ok(Course {
                id: row.get(0)?,
                name: row.get(1)?,
                hours_attended: row.get(2)?,
            })
        })
        .unwrap()
        .map(|c| c.unwrap())
        .collect::<Vec<_>>();

    Ok(warp::reply::json(&courses))
}

pub async fn add_course(course: Course, db: Arc<Database>) -> Result<Json, warp::Rejection> {
    let conn = db.conn.lock().unwrap();
    conn.execute(
        "INSERT INTO courses (name, hours_attended) VALUES (?1, ?2)",
        params![course.name, course.hours_attended],
    )
    .unwrap();

    Ok(warp::reply::json(&course))
}
