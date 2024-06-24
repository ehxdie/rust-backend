use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use db::DB;
use std::convert::Infallible;

// The webframework used to create the server and routing
use warp::{Filter, Rejection};

// Importing modules
mod db;
mod error;
mod handler;

// General Result type
type Result<T> = std::result::Result<T, error::Error>;
// Result type for webresult 
type WebResult<T> = std::result::Result<T, Rejection>;


// For changing data formats, and the debug trait 
#[derive(Debug, Serialize, Deserialize)]
pub struct workout{
    pub title: String,
    pub load: String,
    pub reps: String,
    pub added_at: DateTime<Utc>
}

#[tokio::main]
async fn main() -> Result<String> {
    let db = DB::init().await?;
    let workout = warp::path("api/workouts");

    let workout_routes = workout
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::create_workout)
        .or(workout
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handler::update_workout))
        .or(workout
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handler::delete_workout))
        .or(workout
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handler::get_workout));
        .or(
            workout
            .and(warp::get())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handler::get_one_workout));
        

   
    let routes = workout_routes.recover(error::handle_rejection);

    println!("Started on port 8080");
    warp::serve(routes).run(([0,0,0,0], 8080)).await;
    Ok(());
}

// No idea what this does tbh
fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}