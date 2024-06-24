use crate::parser::post::BlogPost;
use axum::{extract::Path, response::Html};

pub async fn blog(Path(blog_title): Path<String>) -> Html<String> {
    let s = &mut BlogPost::new("Chris Liourtas", "Monday 10pm", blog_title);
    let r = s.render();
    println!("{:#?}", r);
    r
}
