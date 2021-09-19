use std::env;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod tcp_client;
mod tcp_server;
mod udp_server;

fn main() {
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        exit_with("Please specify [tcp|udp] [server|client] [addr:port]");
    }
    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address = &args[3];
    match protocol {
        "tcp" => match role {
            "server" => {
                tcp_server::serve(address).unwrap_or_else(|e| error!("{}", e));
            }
            "client" => {
                tcp_client::connect(address).unwrap_or_else(|e| error!("{}", e));
            }
            _ => exit_with("Please specify server or client on the 2nd argument."),
        },
        "udp" => match role {
            "server" => {
                udp_server::serve(address).unwrap_or_else(|e| error!("{}", e));
            }
            "client" => {
                // todo: implement client
            }
            _ => exit_with("Please specify server or client on the 2nd argument."),
        },
        _ => exit_with("Please specify tcp or udp on the 1st argument."),
    }
}

/// Print error log with passed str and Exit.
fn exit_with(str: &str) {
    error!("{}", str);
    std::process::exit(1);
}
