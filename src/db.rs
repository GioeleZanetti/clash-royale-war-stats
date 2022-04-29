use mongodb::{bson::{doc, Document, Bson}, Collection, Client, Database, options::{ClientOptions}};
use crate::person::Person;
use futures::stream::TryStreamExt;

pub struct MongoClient{}

impl MongoClient{

    pub async fn from_string(connection: String) -> Client {
        let client_options = ClientOptions::parse(connection).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        client
    }

}

pub struct MongoDb{
    client: Client,
    database: Database
}

impl MongoDb{

    pub fn new(client: Client, db: &str) -> Self{
        let database = client.database(db);
        Self{client, database}
    }

    pub async fn list_collections(&self){
        for collection_name in self.database.list_collection_names(None).await.unwrap() {
            println!("{}", collection_name);
        }
    }

    pub async fn find_one_in_collection(&self, collection: &str, filter: Document) -> Document {
        let collection = self.database.collection(collection);
        let document: Document = collection
        .find_one(
            filter,
            None,
        ).await.unwrap().unwrap();
        document
    }

    pub async fn find_in_collection(&self, collection: &str, filter: Document) -> Result<(), mongodb::error::Error>{
        let collection = self.database.collection::<Person>(collection);
        let mut documents = collection
        .find(
            filter,
            None,
        ).await?;
        //TODO: return cursor here
        while let Some(doc) = documents.try_next().await? {
            println!("{:?}", doc)
        }
        Ok(())
    }

}

pub struct Parser{}

impl Parser{

    pub fn deserialize<T>(){
        //TODO: deserialize cursor into T
    }

}