use std::io::Write;
use std::path::PathBuf;
use std::{env, io};

mod client;
mod common;
mod message;
mod server;
mod util;

const HELP: &str = "\
Shopkeeper is a file sharing server and client. It operates on port 6677 by \
                    default. Connections aren't encrypted.

Subcommands:

$ shopkeeper serve
    serve the current directory

$ shopkeeper serve <path>
    serve the specified directory

$ shopkeeper serve <path> <iface>
    serve the specified directory on the given network interface

$ shopkeeper cat <host> <path>
    If the path ends with a slash, list the specified directory on the server. \
                    If it doesn't end with a slash, read the specified file.

$ shopkeeper cat <host>
    List the root directory on the server";

fn main() {
  let mut arguments = env::args().skip(1).filter(|s| !s.starts_with('-'));
  let cmd = arguments.next().expect("Subcommand required");
  if env::args().any(|arg| arg == "--help") {
    return println!("{HELP}");
  }
  let stdout = io::stdout();
  match cmd.as_str() {
    "serve" => {
      let path = (arguments.next())
        .map_or_else(|| env::current_dir().unwrap(), PathBuf::from);
      let iface = arguments.next();
      let log = env::args().filter(|arg| arg == "-v").count();
      let if_str = iface.as_ref().map_or("0.0.0.0", String::as_str);
      server::serve(path, if_str, log);
    },
    "fetch" => {
      let server = arguments.next().expect("Server required");
      let node_str = arguments.next().expect("node ID expected");
      let node = node_str.parse().expect("Node ID must be integer");
      let data = client::fetch(node, &server).expect("Error fetching resource");
      stdout.lock().write_all(&data).unwrap()
    },
    "cat" => {
      let server = arguments.next().expect("Expected server address");
      let resource = arguments.next().map_or_else(
        || client::fetch(0, &server).expect("Error connecting to server"),
        |path| client::cat(&path, &server).expect("Error reading path"),
      );
      stdout.lock().write_all(&resource).unwrap()
    },
    "server_ls" => {
      let path = (arguments.next())
        .map_or_else(|| env::current_dir().unwrap(), PathBuf::from);
      println!("{}", server::ls(&path, &server::tree(&path).1))
    },
    cmd => eprintln!("Unrecognized command {}", cmd),
  }
}
