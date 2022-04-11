mod engine;
mod uci;
mod search;
mod constants;

fn main() {
    let mut weakfish = engine::Weakfish::new();
    weakfish.run();
}