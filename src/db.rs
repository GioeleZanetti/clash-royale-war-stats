use mongodb::{bson::{Document, oid::ObjectId}, Client, Database, options::*, error::Error};
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

    pub async fn find_one_in_collection(&self, collection: &str, filter: Document, options: FindOneOptions) -> Result<Option<Document>, Error> {
        let collection = self.database.collection(collection);
        let document = collection
        .find_one(
            filter,
            options,
        ).await?;
        Ok(document)
    }

    pub async fn find_in_collection<T>(&self, collection: &str, filter: Document, options: FindOptions) -> Result<mongodb::Cursor<T>, Error> {
        let collection = self.database.collection::<T>(collection);
        let documents = collection
        .find(
            filter,
            options,
        ).await?;
        Ok(documents)
    }

    pub async fn insert_into_collection(&self, collection: &str, document: Document) -> Result<Option<ObjectId>, Error> {
        let collection = self.database.collection::<Document>(collection);
        let result = collection.insert_one(
            document, 
            None
        ).await?;
        let id = result.inserted_id.as_object_id();
        Ok(id)
    }

    pub async fn update_from_collection(&self, collection: &str, filter: Document, update: Document) -> Result<u64, Error> {
        let collection = self.database.collection::<Document>(collection);
        let result = collection.update_one(
            filter,
            update, 
            None
        ).await?;
        let count = result.modified_count;
        Ok(count)
    }

    pub async fn delete_from_collection(&self, collection: &str, filter: Document) -> Result<u64, Error> {
        let collection = self.database.collection::<Document>(collection);
        let result = collection.delete_one(
            filter,
            None
        ).await?;
        let count = result.deleted_count;
        Ok(count)
    }

    pub fn to_string(document: &Document) -> String {
        serde_json::to_string_pretty(document).unwrap()
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