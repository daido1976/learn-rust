use std::env;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
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
                // todo: implement server
            }
            "client" => {
                // todo: implement client
            }
            _ => exit_with("Please specify server or client on the 2nd argument."),
        },
        "udp" => match role {
            "server" => {
                // todo: implement server
            }
            "client" => {
                // todo: implement client
            }
            _ => exit_with("Please specify server or client on the 2nd argument."),
        },
        _ => exit_with("Please specify tcp or udp on the 1st argument."),
    }
}

/// Print error with passed str and Exit.
fn exit_with(str: &str) {
    eprintln!("{}", str);
    std::process::exit(1);
}
