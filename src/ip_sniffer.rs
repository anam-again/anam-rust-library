use std::env;
use std::fmt;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{Sender, channel};
use std::thread;

const MAX: u16 = 65535;

struct Arguments {
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        let mut threads: u16 = 4;

        let error = "Error in arguments. Type -h for help";

        let mut i = 1;

        while i < args.len() {
            let arg = &args[i];
            match arg.as_str() {
                "-h" => {
                    return Err("help");
                }
                "-j" => {
                    let t = match args[i + 1].parse::<u16>() {
                        Ok(s) => s,
                        Err(e) => {
                            println!("{}", e);
                            return Err(error);
                        },
                    };
                    threads = t;
                    i += 1;
                }
                _ => {
                    if let Ok(ipaddr) = IpAddr::from_str(&args[i]) {
                        return Ok(Arguments {
                            ipaddr: ipaddr,
                            threads,
                        });
                    }
                }
            }
            i += 1;
        }
        return Err(error);
    }
}

impl fmt::Display for Arguments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IpAddr: {}, Threads: {}", self.ipaddr, self.threads)
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        };

        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}

pub fn ip_sniffer() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = match Arguments::new(&args) {
        Ok(s) => s,
        Err(e) => {
            if e == "help" {
                println!(
                    "Usage: 
                \n \t -h Display this help message
                \n \t -j <u16> Select number of threads
                \n \t IpAddr Input IP Address"
                );
                process::exit(0);
            } else {
                eprintln!("{} problem parsing arguments: {}", program, e);
                process::exit(1);
            }
        }
    };

    println!("{}", arguments);

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }
    for v in out {
        println!("{} is open", v);
    }
    println!("DONE");
}
