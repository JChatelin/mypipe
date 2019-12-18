extern crate clap;
use clap::{Arg, App};
use std::process::Command;

fn main() {
    let matches = App::new("mypipe")
                .version("1.0")
                .author("Joe Chatelin. <josephchatelin@gmail.com>")
                .about("One pipe implementation")
                .arg(Arg::with_name("input")
                    .short("i")
                    .long("--in")
                    .help("This is the input of the pipe")
                    .required(true)
                    .takes_value(true)
                )
                .arg(Arg::with_name("output")
                    .short("o")
                    .long("--out")
                    .help("This is the output of the pipe")
                    .required(true)
                    .takes_value(true)
                )                      
                .get_matches();

    let _input = matches.value_of("input").unwrap_or("hello");
    let _output = matches.value_of("output").unwrap_or("world");

    let cmd_input = Command::new(_input.to_string())
                        .output()
                        .expect("An error occurs during the input process");

    let cmd_output = Command::new(_output.to_string())
                        .arg(String::from_utf8_lossy(&cmd_input.stdout).to_string())
                        .output()
                        .expect("An error occurs during the output process");

    println!("{}", String::from_utf8_lossy(&cmd_output.stdout).to_string());
    
}
