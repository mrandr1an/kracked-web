use orgize::{
    export::{DefaultHtmlHandler, HtmlHandler},
    Element,
};
use slugify::slugify;
use std::{
    io::{Error as IOError, Write, WriterPanicked},
    string::FromUtf8Error,
};

#[derive(Debug)]
enum RenderType {
    CV,
    BlogPost,
    MainPage,
    Wiki,
}

#[derive(Debug)]
enum BlogPostType {}

#[derive(Debug)]
pub enum RenderError {
    IO(IOError),
    Heading,
    TITLE,
    AUTHOR,
    DATE,
    CODEBLOCK,
    TABLE,
    Utf8(FromUtf8Error),
}

impl From<IOError> for RenderError {
    fn from(err: IOError) -> Self {
        RenderError::IO(err)
    }
}

impl From<FromUtf8Error> for RenderError {
    fn from(err: FromUtf8Error) -> Self {
        RenderError::Utf8(err)
    }
}

#[derive(Default)]
pub struct PostHtmlHandler<'a> {
    handler: DefaultHtmlHandler,
    title: &'a str,
    // author: &'a str,
    // date: &'a str,
    // categories: &'a str,
    // tags: &'a str,
}

impl<'a> HtmlHandler<RenderError> for PostHtmlHandler<'a> {
    fn start<W: Write>(&mut self, mut w: W, element: &Element) -> Result<(), RenderError> {
        match element {
            Element::Title(title) => {
                if title.level == 1 {
                    Ok(write!(
                        w,
                        "<div class=\"post_start\"><div id=\"title_container\"><h1 id=\"title\">"
                    )?)
                } else {
                    Ok(write!(
                        w,
                        "<div class=\"section_start\"><div class=\"section_container\"><h{}>",
                        title.level,
                    )?)
                }
            }
            e => Ok(self.handler.start(w, e)?),
        }
    }

    fn end<W: Write>(&mut self, mut w: W, element: &Element) -> Result<(), RenderError> {
        match element {
            Element::Title(title) => Ok(write!(w, "</h{}></div></div>", title.level)?),
            e => Ok(self.handler.end(w, e)?),
        }
    }
}
