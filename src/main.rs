use std::{error::Error, io::Write};
use std::io::{self, Read};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use clap::{App, AppSettings, Arg};

fn main() {
    let matches = App::new("Line echo back terminal")
        .about("Displays echo back from the serial port in lines.")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("port")
             .help("The device path to a serial port")
             .use_delimiter(false)
             .required(true))
        .get_matches();
    let port_name = matches.value_of("port").unwrap();

    let exit_code = match run(&port_name, 115200) {
        Ok(_) => 0,
        Err(e) => {
            println!("Error: {}", e);
            1
        }
    };

    std::process::exit(exit_code);
}

fn run(port_name: &str, baud_rate: u32) -> Result<(), Box<dyn Error>> {
    let mut port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open()
        .map_err(|ref e| format!("Port '{}' not available: {}", &port_name, e))?;

    let chan_clear_buf = input_service();

    println!("Connected to {} at {} baud", &port_name, &baud_rate);
    println!("Ctrl+D (*nix) or Ctrl+Z (Win) to stop.");

    loop {
        match chan_clear_buf.try_recv() {
            Ok(buf) => {
                match port.write(&buf) {
                    Ok(_) => print!("echo: "),
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => panic!("Error while writing data to the port: {}", e),
                };
            }
            Err(mpsc::TryRecvError::Empty) => (),
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("Stopping.");
                break;
            }
        }

        let mut buf = [0; 1];
        match port.read(&mut buf) {
            Ok(_) => io::stdout().write_all(&buf).unwrap(),
            Err(_e) => {},
        }
    }

    Ok(())
}

fn input_service() -> mpsc::Receiver<Vec<u8>> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            let mut buffer: Vec<u8> = vec![0; 1024];
            // Block awaiting any user input
            match io::stdin().read(buffer.as_mut_slice()) {
                Ok(0) => {
                    drop(tx); // EOF, drop the channel and stop the thread
                    break;
                }
                Ok(_) => tx.send(buffer).unwrap(), // send bytes
                Err(e) => panic!(e),
            }
        }
    });

    rx
}
