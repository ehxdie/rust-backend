
use crate::{error::Error::*, handler::workoutRequest, workout, Result};
use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson, DateTime as MongoDateTime};
use mongodb::{options::ClientOptions, Client, Collection};

const DB_NAME: &str = "test";
const COLL: &str = "workouts";

const ID: &str = "_id";
const TITLE: &str = "name";
const LOAD: &str = "author";
const REPS: &str = "num_pages";
// const ADDED_AT: &str = "added_at";


#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> Result<Self> {
        let mut client_options = ClientOptions::parse("mongodb+srv://ephraim:ephraim@mern.6tvgmwx.mongodb.net/?retryWrites=true&w=majority&appName=Mern").await?;
        client_options.app_name = Some("test".to_string());

        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }

    pub async fn fetch_workouts(&self) -> Result<Vec<workout>> {
        let mut cursor = self
            .get_collection()
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;

        let mut result: Vec<workout> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.doc_to_book(&doc?)?);
        }
        Ok(result)
    }

   pub async fn fetch_one_workout(&self, id: &str) -> Result<Option<workout>> {
        let oid = ObjectId::parse_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let filter = doc! {
            "_id": oid,
        };

        let doc = self
            .get_collection()
            .find_one(filter, None)
            .await
            .map_err(MongoQueryError)?;

        if let Some(doc) = doc {
            Ok(Some(self.doc_to_book(&doc)?))
        } else {
            Ok(None)
        }
    }

    pub async fn create_workout(&self, entry: &workoutRequest) -> Result<()> {
        let doc = doc! {
            TITLE: entry.title.clone(),
            LOAD: entry.load.clone(),
            REPS: entry.reps.clone(),
            // ADDED_AT: Utc::now(),
            };

        self.get_collection()
            .insert_one(doc, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn update_workout(&self, id: &str, entry: &workoutRequest) -> Result<()> {
        let oid = ObjectId::parse_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            "_id": oid,
        };
        let doc = doc! {
            TITLE: entry.title.clone(),
            LOAD: entry.load.clone(),
            REPS: entry.reps.clone(),
            // ADDED_AT: Utc::now(),
        };

        self.get_collection()
            .update_one(query, doc, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn delete_workout(&self, id: &str) -> Result<()> {
        let oid = ObjectId::parse_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let filter = doc! {
            "_id": oid,
        };

        self.get_collection()
            .delete_one(filter, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    fn get_collection(&self) -> Collection<Document> {
        self.client.database(DB_NAME).collection(COLL)
    }

    fn doc_to_book(&self, doc: &Document) -> Result<workout> {
        let id = doc.get_object_id(ID)?;
        let title = doc.get_str(TITLE)?;
        let load = doc.get_str(LOAD)?;
        let reps = doc.get_str(REPS)?;
        // let added_at = doc.get_datetime(ADDED_AT)?;
        



        let workout = workout {
            id: id.to_hex(),
            title: title.to_owned(),
            load: load.to_owned(),
            reps: reps.to_owned(),
            // added_at: *added_at,
           
        };
        Ok(workout)
    }
}