use rocket::serde::{
    json::{json, Json, Value},
    Deserialize,
};
use uuid::Uuid;

use crate::{db, metadata_extractor as meta_ex};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ImageData<'r> {
    name: &'r str,
    width: u32,
    height: u32,
    data: &'r str,
}

// @desc        Returns Index Page for Localhost
// @route       GET /
// @access      Public
#[get("/")]
pub fn index() -> Value {
    json!({
        "_message": "Welcome to Image Extractor API",
        "GET /list": "Get list of all the images",
        "GET /<id>": "Get Image metadata with a particular ID",
        "POST /upload": "Upload image and get its ID with metadata",
    })
}

// @desc        Uploads image, returns meatdata as result
// @route       POST /upload
// @access      Public
#[post("/upload", data = "<body>")]
pub fn upload(body: Json<ImageData<'_>>) -> Value {
    let id = Uuid::new_v4().to_string();
    let metadata = match meta_ex::regenerate(body.data, body.name, body.width, body.height) {
        Ok(result) => result,
        Err(err) => {
           println!("Metadata fetch failed! - \n{:#?}", err);
            json!({"error": err.to_string()})
        }
    };

    let result = db::insert(&id, body.name, metadata.to_string().as_str());
    match result {
        Ok(result) => result,
        Err(err) => {
            println!("DB insert operation resulted in error! - \n{:#?}", err);
            json!({
                "error": err.to_string()
            })
        }
    }
}

// @desc        Get Image by ID
// @route       GET /<id>
// access       Public
#[get("/<id>")]
pub fn find(id: &str) -> Value {
    let result = db::select(id);
    if result.is_ok() {
        result.unwrap()
    } else {
        println!("Error - {:?}", result.unwrap_err());
        json!({
            "error": "Some error occured while fetching the result"
        })
    }
}

// @desc        Get ImageID as a list (without metadata)
// @route       GET /list
// access       Public
#[get("/list")]
pub fn list() -> Value {
    let result = db::select_all();
    if result.is_ok() {
        result.unwrap()
    } else {
        println!("Error - {:?}", result.unwrap_err());
        json!({
            "error": "Some error occured while fetching the result"
        })
    }
}
