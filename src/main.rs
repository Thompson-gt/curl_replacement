//local modules
#[path = "./displayers/mod.rs"]
pub mod displayers;
#[path = "./formatters/mod.rs"]
pub mod formatters;
#[path = "./handlers/mod.rs"]
pub mod handlers;
use handlers::handler::run_program;

fn main() {
    run_program();
}
