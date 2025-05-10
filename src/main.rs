mod app;
mod components;
mod systems;
mod resources;
mod utils;

use app::RegolithApp;

fn main() {
    env_logger::init();
    let mut app = RegolithApp::new();
    app.run();
}