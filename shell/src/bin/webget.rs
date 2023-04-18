use openssl::ssl::{SslConnector, SslMethod};
use std::env::args;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::net::TcpStream;
use std::io::Write;
use std::str::FromStr;


fn main() -> anyhow::Result<()>{
    openssl_probe::init_ssl_cert_env_vars();
    for arg in args().skip(1) {
        match cmd_processing(arg.as_str()) {
            Ok(_) => {}
            Err(e) => println!("Error: {e}")
        }
    }
    Ok(())
}

fn cmd_processing(input: &str) -> anyhow::Result<()>{
    let components = input.split("/").collect::<Vec<_>>();
    let sub_component = components[2].split(':').collect::<Vec<_>>();
    let mut port = 0;
    if sub_component.len() == 2 {
        port = FromStr::from_str(sub_component[1]).unwrap();
    } else {
        port = 443;
    }
    let host = sub_component[0];
    if components.len() == 2 {
        
    }
    let mut file = String::new();
    for i in 3..components.len() {
        file.push('/');
        file.push_str(components[i]);
    }
    let mut filelocation = String::new();
    let filelocation = components[components.len()-1];
    get_create(host, port, file.as_str(), filelocation)
}

fn get_create(host: &str, port: usize, request: &str, filelocation: &str) -> anyhow::Result<()> {
    println!("Connecting");
    println!("Host: {}", host);
    println!("Port: {}", port);
    println!("File: {}", request);
    let tcp = TcpStream::connect(format!("{}:{}", host, port))?;
    println!("TCP Generated: {}:{}", host, port);
    let connector = SslConnector::builder(SslMethod::tls())?.build();
    println!("Connector Generated");
    let mut stream = connector.connect(host, tcp).unwrap();
    println!("Stream Generated: {}, {}:{}", host, host, port );

    let http_request = format!("GET {} HTTP/1.1\r\nHost: {}\r\nConnection: Close\r\n\r\n", request, host);
    println!("{}", http_request);
    stream.write_all(http_request.as_bytes())?;

    let buffer = BufReader::new(stream);
    let mut file = File::create(filelocation)?;
    for line in buffer.lines() {
        let line = line?;
        println!("{}", line);
        file.write_all(line.as_bytes())?;
        file.write(b"\n")?;
    }
    println!("Downloaded file to: {}", filelocation);
    Ok(())
}

