use std::{
    fs::{File, OpenOptions},
    io::{Bytes, Error as IOError, Write},
    path::Path,
};

use axum::{
    extract::{multipart::MultipartError, Multipart},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct BlogPost {
    title: String,
    content: String,
}

#[derive(Debug)]
pub enum PublishError {
    IO(IOError),
    MultipartError(MultipartError),
    AlreadyExists(String),
}

impl IntoResponse for PublishError {
    fn into_response(self) -> axum::response::Response {
        match self {
            PublishError::IO(e) => panic!("{}", e),
            PublishError::MultipartError(e) => panic!("{}", e),
            PublishError::AlreadyExists(e) => panic!("File with {} already exists", e),
            unknown => todo!("Unknown Error: {:#?}", unknown),
        }
    }
}

impl From<IOError> for PublishError {
    fn from(value: IOError) -> Self {
        PublishError::IO(value)
    }
}

impl From<MultipartError> for PublishError {
    fn from(value: MultipartError) -> Self {
        PublishError::MultipartError(value)
    }
}

pub async fn publish_blog_handler(mut multipart: Multipart) -> Result<String, PublishError> {
    while let Some(field) = multipart.next_field().await? {
        let file_name = field.file_name().unwrap_or("default").to_string();
        let file_content = field.bytes().await?;

        let mut project_root = std::env::current_dir()?;
        project_root.push("blog/");
        project_root.push(&file_name);
        let publish_dir = project_root;

        if Path::new(&publish_dir).exists() {
            return Err(PublishError::AlreadyExists(file_name));
        } else {
            println!("{}", publish_dir.to_str().unwrap());
            let mut file_instance = OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open(&publish_dir)?;
            let _ = file_instance.write(&file_content)?;
        }
    }
    Ok(String::from("Published!\n"))
}
