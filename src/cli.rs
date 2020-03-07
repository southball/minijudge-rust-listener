use clap::Clap;

/// MiniJudge-Rust-Listener
/// A socket listener for MiniJudge-Rust.
#[derive(Clap, Clone)]
#[clap(version = "0.0-alpha.1", author = "Southball")]
pub struct Opts {
    /// Socket to listen from.
    #[clap(long = "socket")]
    pub socket: String,
}
