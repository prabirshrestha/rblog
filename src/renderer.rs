pub trait Render {
    fn render<Call>(&mut self, call: Call) -> std::io::Result<()>
    where
        Call: FnOnce(&mut dyn std::io::Write) -> std::io::Result<()>;
}

impl Render for tide::Response {
    fn render<Call>(&mut self, call: Call) -> std::io::Result<()>
    where
        Call: FnOnce(&mut dyn std::io::Write) -> std::io::Result<()>,
    {
        let mut buf = vec![];
        call(&mut buf)?;
        self.set_body(buf);
        Ok(())
    }
}
