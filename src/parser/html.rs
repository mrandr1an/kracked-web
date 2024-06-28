use super::theme::Theme;
use orgize::{
    export::{DefaultHtmlHandler, HtmlHandler},
    Element,
};
use std::{
    io::{Error as IOError, Write},
    string::FromUtf8Error,
};

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
    theme: Theme,
}

impl<'a> BlogPostHtmlHandler<'a> {
    pub fn new(title: &'a str, author: &'a str, date: &'a str) -> Self {
        Self {
            handler: DefaultHtmlHandler,
            title,
            author,
            date,
            theme: Theme::new(),
        }
    }
}

impl<'a> HtmlHandler<RenderError> for BlogPostHtmlHandler<'a> {
    fn start<W: Write>(&mut self, mut w: W, element: &Element) -> Result<(), RenderError> {
        match element {
            Element::Headline { level } => {
                if *level == 1 {
                    Ok(write!(w, "{}", self.theme.title_start())?)
                } else {
                    Ok(write!(w, "{}", self.theme.heading_start(),)?)
                }
            }
            Element::Document { .. } => Ok(write!(w, "{}", self.theme.document_start())?),
            Element::SourceBlock(s) => {
                let codeblock = self.theme.codeblock_start().replace("#lang#", &s.language);
                Ok(write!(w, "{}{}", codeblock, s.contents)?)
            }
            e => Ok(self.handler.start(w, e)?),
        }
    }

    fn end<W: Write>(&mut self, mut w: W, element: &Element) -> Result<(), RenderError> {
        match element {
            Element::Headline { level } => {
                if *level == 1 {
                    Ok(write!(w, "{}", self.theme.title_end())?)
                } else {
                    Ok(write!(w, "{}", self.theme.heading_end(),)?)
                }
            }
            Element::Document { .. } => Ok(write!(w, "{}", self.theme.document_end())?),
            Element::SourceBlock(_) => Ok(write!(w, "{}", self.theme.codeblock_end())?),
            e => Ok(self.handler.end(w, e)?),
        }
    }
}
