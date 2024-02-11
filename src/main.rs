//! Miimal HTTP server
//!
//! # Architecture
//! - TCP Listener
//! - Http Parser
//! - Handler

use server::Server;
mod http;
mod server;
fn main() {
    // Create a server
    let server = Server::new("127.0.0.1:8080");
    // run the server
    let _ = server.run();
}
