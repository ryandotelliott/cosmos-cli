mod cli;
mod libs;

fn main() {
    dotenvy::dotenv().ok();
    cli::commands::main::main_menu();
}
