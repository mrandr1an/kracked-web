use std::{env, fs, path::PathBuf};

use super::html::*;
use axum::response::Html;
use orgize::Org;

pub struct BlogPost<'a> {
    contents: String,
    author: &'a str,
    title: String,
    date: &'a str,
}

impl<'a> BlogPost<'a> {
    fn get_file_contents_from_title(file_name: String) -> String {
        let current_dir = env::current_dir().expect("Failed to get current directory");

        #[allow(clippy::useless_conversion)]
        let mut file_path = PathBuf::from(current_dir);
        file_path.push("src/posts");
        file_path.push(file_name + ".org");

        match fs::read_to_string(&file_path) {
            Ok(contents) => contents,
            Err(err) => err.to_string(),
        }
    }

    pub fn new(author: &'a str, date: &'a str, title: String) -> Self {
        Self {
            contents: Self::get_file_contents_from_title(title.clone()),
            author,
            title,
            date,
        }
    }

    pub fn render(&mut self) -> Html<String> {
        let mut writer = Vec::new();
        let mut handler = BlogPostHtmlHandler::new(self.author, &self.title, self.date);
        let _ = Org::parse(&self.contents).write_html_custom(&mut writer, &mut handler);
        Html::from(String::from_utf8(writer).expect("could not convert utf8 to string"))
    }
}
