use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use mongodb::bson::extjson::de::Error;
use mongodb::results::InsertOneResult;

use super::MongoRepo;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct User {
    pub username: String,
    pub password: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>
}

impl MongoRepo {
    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error>{
        todo!()
    }  
}
