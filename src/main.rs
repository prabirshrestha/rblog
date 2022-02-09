use rblog::app::app;

fn main() {
    pretty_env_logger::init();
    trillium_tokio::run(app());
}
