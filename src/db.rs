use mongodb::{bson::{Document, oid::ObjectId}, Client, Database, options::{ClientOptions}};
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
    database: Database
}

impl MongoDb{

    pub fn new(client: Client, db: &str) -> Self{
        let database = client.database(db);
        Self{ database}
    }

    pub async fn list_collections(&self){
        for collection_name in self.database.list_collection_names(None).await.unwrap() {
            println!("{}", collection_name);
        }
    }

    pub async fn find_one_in_collection(&self, collection: &str, filter: Document) -> Result<Option<Document>, mongodb::error::Error> {
        let collection = self.database.collection(collection);
        let document = collection
        .find_one(
            filter,
            None,
        ).await?;
        Ok(document)
    }

    pub async fn find_in_collection(&self, collection: &str, filter: Document) -> Result<mongodb::Cursor<Document>, mongodb::error::Error> {
        let collection = self.database.collection::<Document>(collection);
        let documents = collection
        .find(
            filter,
            None,
        ).await?;
        Ok(documents)
    }

    pub async fn insert_into_collection(&self, collection: &str, document: Document) -> Result<Option<ObjectId>, mongodb::error::Error> {
        let collection = self.database.collection::<Document>(collection);
        let result = collection.insert_one(
            document, 
            None
        ).await?;
        let id = result.inserted_id.as_object_id();
        Ok(id)
    }

    pub async fn update_from_collection(&self, collection: &str, filter: Document, update: Document) -> Result<Option<u64>, mongodb::error::Error> {
        let collection = self.database.collection::<Document>(collection);
        let result = collection.update_one(
            filter,
            update, 
            None
        ).await?;
        let id = result.modified_count;
        Ok(Some(id))
    }

    pub async fn delete_from_collection(&self, collection: &str, filter: Document) -> Result<Option<u64>, mongodb::error::Error> {
        let collection = self.database.collection::<Document>(collection);
        let result = collection.delete_one(
            filter,
            None
        ).await?;
        let count = result.deleted_count;
        Ok(Some(count))
    }

}

pub struct Parser{}

impl Parser{

    pub async fn print_find_result(mut cursor: mongodb::Cursor<Document>){
        while let Some(element) = cursor.try_next().await.unwrap() {
            println!("{}", element)
        }
    }

}