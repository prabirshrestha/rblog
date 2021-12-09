use rblog::app::app;

fn main() {
    pretty_env_logger::init();
    trillium_smol::run(app());
}
