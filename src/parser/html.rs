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
enum BlogPostType {}

#[derive(Debug)]
pub enum RenderError {
    IO(IOError),
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
pub struct BlogPostHtmlHandler<'a> {
    handler: DefaultHtmlHandler,
    title: &'a str,
    author: &'a str,
    date: &'a str,
    // categories: &'a str,
    // tags: &'a str,
}

impl<'a> BlogPostHtmlHandler<'a> {
    pub fn new(title: &'a str, author: &'a str, date: &'a str) -> Self {
        Self {
            handler: DefaultHtmlHandler,
            title,
            author,
            date,
        }
    }
}

impl<'a> HtmlHandler<RenderError> for BlogPostHtmlHandler<'a> {
    fn start<W: Write>(&mut self, mut w: W, element: &Element) -> Result<(), RenderError> {
        match element {
            Element::Title(title) => {
                if title.level == 1 {
                    Ok(write!(
                        w,
                        "<div class=\"section_start\"><div class=\"section_container\"><h2>",
                    )?)
                } else {
                    Ok(write!(
                        w,
                        "<div class=\"section_start\"><div class=\"section_container\"><h{}>",
                        title.level + 1,
                    )?)
                }
            }
            Element::Document { .. } => Ok(write!(
                w,
                "<div class=\"post_start\"><div id=\"title_container\">{0}\n{1}\n{2}",
                self.author, self.date, self.title,
            )?),
            e => Ok(self.handler.start(w, e)?),
        }
    }

    fn end<W: Write>(&mut self, mut w: W, element: &Element) -> Result<(), RenderError> {
        match element {
            Element::Title(title) => Ok(write!(w, "</h{}></div></div>", title.level + 1)?),
            Element::Document { .. } => Ok(write!(w, "</div></div>")?),
            e => Ok(self.handler.end(w, e)?),
        }
    }
}
