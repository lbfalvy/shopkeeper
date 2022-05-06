use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::io;

mod message;
mod server;
mod common;
mod client;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argprims: Vec<&str> = args.iter().map(String::as_str).collect();
    let default_iface = "0.0.0.0:6677";
    let stdout = io::stdout();
    match argprims[..] {
        [_, "serve", pathstr, interface] => server::serve(PathBuf::from(pathstr), interface),
        [_, "serve", pathstr] => server::serve(PathBuf::from(pathstr), default_iface),
        [_, "serve"] => server::serve(env::current_dir().unwrap(), default_iface),
        [_, "fetch", server, node] => match node.parse::<u32>() {
            Err(_) => eprintln!("Node not u32"),
            Ok(code) => match client::fetch(code, server) {
                None => eprintln!("Error fetching resource {} from {}", code, server),
                Some(data) => stdout.lock().write_all(&data).unwrap()
            }
        },
        [_, "cat", server, resoource_path] => match client::cat(resoource_path, server) {
            None => eprintln!("Error reading path {} on {}", resoource_path, server),
            Some(data) => stdout.lock().write_all(&data).unwrap()
        },
        [_, "cat", server] => match client::fetch(0, server) {
            None => eprintln!("Failed to connect to the server at {}", server),
            Some(data) => stdout.lock().write_all(&data).unwrap()
        }
        [_, "server_ls", pathstr] => {
            let path = PathBuf::from(pathstr);
            let (_, indices) = server::tree(&path);
            println!("{}", server::ls(&path, &indices));
        },
        [_, "server_ls"] => {
            let (_, indices) = server::tree(&env::current_dir().unwrap());
            println!("{}", server::ls(&env::current_dir().unwrap(), &indices));
        },
        [_, cmd, ..] => eprintln!("Unrecognized command {}", cmd),
        _ => eprintln!("Command required")
    }
}