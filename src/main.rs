pub mod cli;
pub mod grid;
pub mod seed;

fn main() -> std::io::Result<()> {
    cli::run()
}
