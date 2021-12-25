use env_logger::Env;
use log::info;

mod engine;
mod scene;

fn main() {
  let env = Env::default()
    .filter_or("MY_LOG_LEVEL", "pong")
    .write_style_or("MY_LOG_STYLE", "info");

  env_logger::init_from_env(env);

  info!("Starting pong");
  pollster::block_on(engine::run());
}