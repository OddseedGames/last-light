mod app;
mod audio;
mod browser;
mod view;

use app::config;

#[macroquad::main(config)]
async fn main() {
    app::run().await;
}
