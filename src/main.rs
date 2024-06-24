use chrono::prelude::*;
pub struct workout{
    pub title: String,
    pub load: String,
    pub reps: String,
    pub added_at: DateTime<Utc>
}

fn main() {
    println!("Hello, world!");
}
