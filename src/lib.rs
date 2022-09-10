use anyhow::Result;
use salvo::prelude::*;

pub mod appstate;
pub mod blog;
pub mod markdown;
pub mod routes;
pub mod server;

pub fn render<F>(res: &mut Response, do_render: F) -> Result<&mut Response>
where
    F: FnOnce(&mut Vec<u8>) -> std::io::Result<()>,
{
    let mut buf = Vec::new();
    do_render(&mut buf)?;
    res.render(Text::Plain(String::from_utf8(buf)?));
    Ok(res)
}

pub fn render_html<F>(res: &mut Response, do_render: F) -> Result<&mut Response>
where
    F: FnOnce(&mut Vec<u8>) -> std::io::Result<()>,
{
    let mut buf = Vec::new();
    do_render(&mut buf)?;
    res.render(Text::Html(String::from_utf8(buf)?));
    Ok(res)
}

pub fn render_rss<F>(res: &mut Response, do_render: F) -> Result<&mut Response>
where
    F: FnOnce(&mut Vec<u8>) -> std::io::Result<()>,
{
    let mut buf = Vec::new();
    do_render(&mut buf)?;
    res.render(Text::Rss(String::from_utf8(buf)?));
    Ok(res)
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
