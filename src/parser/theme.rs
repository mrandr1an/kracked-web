//! This module consists of everything neccesary for generating the appropriate CSS (theme)
//! of the krackec-org website

use orgize::export::DefaultHtmlHandler;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Theme {
    #[serde(rename = "theme")]
    name: String,
    #[serde(rename = "blogpost")]
    blog_post_theme: BlogPostTheme,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct BlogPostTheme {
    name: String,
    title: Title,
    document: Document,
    heading: Heading,
    codeblock: Codeblock,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Title {
    author: Option<Author>,
    date: Option<Date>,
    start: String,
    end: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Date {
    start: String,
    end: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Author {
    start: String,
    end: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Heading {
    start: String,
    end: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Codeblock {
    start: String,
    end: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Document {
    start: String,
    end: String,
}

impl Theme {
    pub fn new() -> Self {
        let current_dir = env::current_dir().expect("Failed to get current directory");

        #[allow(clippy::useless_conversion)]
        let mut file_path = PathBuf::from(current_dir);
        file_path.push("src/theme");
        file_path.push("theme.toml");

        match fs::read_to_string(&file_path) {
            Ok(contents) => toml::from_str(&contents).unwrap(),
            Err(err) => panic!("Panic: {}", err),
        }
    }

    pub fn title_start(&self) -> String {
        self.blog_post_theme.title.start.clone()
    }

    pub fn title_end(&self) -> String {
        self.blog_post_theme.title.end.clone()
    }

    pub fn author_start(&self) -> String {
        match &self.blog_post_theme.title.author {
            Some(author) => author.start.clone(),
            None => String::from(""),
        }
    }

    pub fn author_end(&self) -> String {
        match &self.blog_post_theme.title.author {
            Some(author) => author.end.clone(),
            None => String::from(""),
        }
    }

    pub fn date_start(&self) -> String {
        match &self.blog_post_theme.title.date {
            Some(date) => date.start.clone(),
            None => String::from(""),
        }
    }

    pub fn date_end(&self) -> String {
        match &self.blog_post_theme.title.date {
            Some(date) => date.end.clone(),
            None => String::from(""),
        }
    }

    pub fn heading_start(&self) -> String {
        self.blog_post_theme.heading.start.clone()
    }

    pub fn heading_end(&self) -> String {
        self.blog_post_theme.heading.end.clone()
    }

    pub fn codeblock_start(&self) -> String {
        self.blog_post_theme.codeblock.start.clone()
    }

    pub fn codeblock_end(&self) -> String {
        self.blog_post_theme.codeblock.end.clone()
    }

    pub fn document_start(&self) -> String {
        self.blog_post_theme.document.start.clone()
    }

    pub fn document_end(&self) -> String {
        self.blog_post_theme.document.end.clone()
    }
}
