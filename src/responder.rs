use rocket::response::{Responder};
use rocket::http::{Header, ContentType};

#[derive(Responder)]
pub struct JsonResponder {
    inner: String,
    content_type: ContentType,
    cors: Header<'static>,
}

impl JsonResponder{

    pub fn new(inner: String) -> Self {
        let content_type = ContentType::JSON;
        let cors = Header::new("Access-Control-Allow-Origin", "*");
        Self{inner, content_type, cors}
    }

}