use clap::Parser;
use std::path::PathBuf;

use super::verify_path;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(name = "serve", about = "serve a directory over HTTP")]
    Serve(HttpServeOps),
}

#[derive(Debug, Parser)]
pub struct HttpServeOps {
    #[arg(short, long, value_parser=verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
