
use crate::{db::DB, WebResult};
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reject, reply::json, Reply};

#[derive(Serialize, Deserialize, Debug)]
pub struct workoutRequest{
    pub title: String,
    pub load: String,
    pub reps: String,
}


pub async fn get_workout(db: DB) -> WebResult<impl Reply> {
    let workouts = db.fetch_workouts().await.map_err(|e| reject::custom(e))?;
    Ok(json(&workouts))
}

pub async fn get_one_workout(id: String, db: DB) -> WebResult<impl Reply> {
    let workout = db.fetch_one_workout(&id)
    .await.map_err(|e| reject::custom(e))?;
    Ok(json(&workout))
}

// pub async fn create_workout(body: workoutRequest, db: DB) -> WebResult<impl Reply> {
//     db.create_workout(&body).await.map_err(|e| reject::custom(e))?;
//     Ok(StatusCode::CREATED)
// }
pub async fn create_workout(workout: workoutRequest, db: DB) -> WebResult<impl Reply> {
    match db.create_workout(&workout).await {
        Ok(created_workout) => Ok(warp::reply::json(&created_workout)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn update_workout(id: String, body: workoutRequest, db: DB) -> WebResult<impl Reply> {
    db.update_workout(&id, &body)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

pub async fn delete_workout(id: String, db: DB) -> WebResult<impl Reply> {
    let workout = db.delete_workout(&id).await.map_err(|e| reject::custom(e))?;
    Ok(json(&workout))
}