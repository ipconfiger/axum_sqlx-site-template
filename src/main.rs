use crate::app::start_serve;
use crate::cli::run_command;
use crate::conf::{config_from_matches, get_matches};

mod cli;
mod service;
mod modals;
mod conf;
mod app;
mod handlers;

#[tokio::main]
async fn main() {
    let matches = get_matches();
    let config = config_from_matches(&matches);
    if let Some(cmd) = matches.get_one::<String>("cmd") {
        run_command(cmd.as_str(), &config).await;
    }else{
        if matches.is_present("serve") {
            start_serve(&config).await;
        }else{
            println!("Present --serve to start server, or --cmd to run command");
        }
    }
}