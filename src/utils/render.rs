use anyhow::Result;
use salvo::prelude::*;

pub trait RenderExt {
    fn render<F>(&mut self, do_render: F) -> Result<&mut Response>
    where
        F: FnOnce(&mut Vec<u8>) -> std::io::Result<()>;

    fn render_html<F>(&mut self, do_render: F) -> Result<&mut Response>
    where
        F: FnOnce(&mut Vec<u8>) -> std::io::Result<()>;
}

impl RenderExt for Response {
    fn render<F>(&mut self, do_render: F) -> Result<&mut Response>
    where
        F: FnOnce(&mut Vec<u8>) -> std::io::Result<()>,
    {
        let mut buf = Vec::new();
        do_render(&mut buf)?;
        self.render(Text::Plain(String::from_utf8(buf)?));
        Ok(self)
    }

    fn render_html<F>(&mut self, do_render: F) -> Result<&mut Response>
    where
        F: FnOnce(&mut Vec<u8>) -> std::io::Result<()>,
    {
        let mut buf = Vec::new();
        do_render(&mut buf)?;
        self.render(Text::Html(String::from_utf8(buf)?));
        Ok(self)
    }
}
