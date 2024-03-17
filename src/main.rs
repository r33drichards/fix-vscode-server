use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::os::unix::fs as unix_fs;
use std::process::Command;

fn main() {
    let nix_node_path = env::args()
        .nth(1)
        .expect("Error: No argument provided. Please provide a node path as the argument.");

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                if line.contains("stderr")
                    && line.contains("node: cannot execute: required file not found")
                {
                    let node_path_from_line = line.split(' ').find(|&x| x.contains("/node"));

                    match node_path_from_line {
                        Some(node_path) => {
                            // Trim potential trailing characters
                            let node_path = node_path.trim_end_matches(':');

                            // Remove the existing node path
                            if let Err(e) = fs::remove_file(node_path) {
                                eprintln!("Error removing the node file: {}", e);
                                continue;
                            }

                            // Create a symlink to the provided nix_node_path
                            if let Err(e) = unix_fs::symlink(&nix_node_path, node_path) {
                                eprintln!("Error creating symlink: {}", e);
                                continue;
                            }

                            // // Restart the vscode-server systemd unit
                            // if let Err(e) = Command::new("systemctl")
                            //     .args(["restart", "vscode-server.service"])
                            //     .output()
                            // {
                            //     eprintln!("Failed to restart vscode-server: {}", e);
                            // }
                            // // Restart the vscode-server systemd unit
                            // if let Err(e) = Command::new("systemctl")
                            //     .args(["restart", "fix-vscode-server.service"])
                            //     .output()
                            // {
                            //     eprintln!("Failed to restart vscode-server: {}", e);
                            // }

                            // Additional systemd unit restarts can be handled similarly
                        }
                        None => eprintln!("Node path not found in the provided line."),
                    }
                } else {
                    println!("{}", line);
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        }
    }
}
