use std::{env, io, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Please only provide one argument!");
    }

    let mode = match args[1].as_str() {
        "P" => pnchcrd::Mode::Punc,
        "C" => pnchcrd::Mode::Calc,
        _ => panic!("Incorrect mode syntax! Use P for punch mode and C for calc mode!"),
    };

    let ports = serialport::available_ports().expect("No ports found");
    let mut use_port = String::new();

    for i in ports {
        println!("{}", i.port_name);
        if i.port_name.len() == 12 {
            if i.port_name[8..11].to_string() == "ACM".to_string() {
                use_port = i.port_name;
            }
        }
    }

    loop {
        let mut port = serialport::new(use_port.as_str(), 9600)
            .timeout(Duration::from_millis(1000))
            .open()
            .expect("Failed to open port");

        let mut serial_buf: Vec<u8> = vec![0; 32];
        match port.read(serial_buf.as_mut_slice()) {
            Ok(bytes_read) if bytes_read > 0 => {
                match String::from_utf8(serial_buf[..bytes_read].to_vec()) {
                    Ok(string) => {
                        //THIS IS WHERE ALL THE ACTUAL CODE HAPPENS
                        let name =
                            pnchcrd::punch_entry(string.trim().to_string(), mode.clone()).unwrap();
                        let mut combine = String::new();
                        for i in name {
                            combine.push_str(i.as_str());
                            combine.push('|');
                        }
                        combine.pop();
                        port.write(combine.as_bytes()).unwrap();
                    }
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {}
            Err(e) => {
                eprintln!("{:?}", e);
                break;
            }
            Ok(_) => {}
        }
    }
}
