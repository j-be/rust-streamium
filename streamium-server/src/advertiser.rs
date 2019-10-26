use std::io::Write;
use std::net::{SocketAddr, TcpStream, UdpSocket};

use ifaces::Kind;
use ifaces::NextHop::Broadcast;
use quick_xml::events::Event;
use quick_xml::Reader;

pub fn listen(server_ip: &str, server_port: u16) {
    loop {
        let name = "RustStreamium";

        let socket = UdpSocket::bind(format!("{}:{}", get_broadcast(server_ip), server_port))
            .expect("Could not bind to address");

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 256];
        let (_, src) = socket.recv_from(&mut buf)
            .expect("Error while receiving!");

        let request = String::from_utf8(buf.to_vec())
            .expect("Error writing buffer to String!");
        println!("Got {:?}", request);

        let response = format!("<PCLinkServer><Version>1.0</Version><VendorID>MUSICMATCH</VendorID><name>{}</name><ShortName>{}</ShortName><IP>{}</IP><Port>{}</Port></PCLinkServer>",
                               name, name, ip_to_streamium(server_ip), port_to_streamium(server_port));

        let response_port_streamium: u16 = get_port_from_xml(request)
                .map(|s| s.parse::<u16>()
                    .unwrap_or_else(|_| 0))
                .unwrap_or_else(|_| 0);

        if response_port_streamium > 0 {
            let response_addr = format!("{}:{}", src.ip(), streamium_to_port(response_port_streamium));
            println!("Sending {} to {}", response, response_addr);
            TcpStream::connect(response_addr)
                .and_then(|mut s| s.write(response.as_bytes()))
                .unwrap_or_else(|_| {
                    println!("Error while sending response!");
                    0
                });
        }
    } // the socket is closed here
}

fn get_broadcast(ip: &str) -> String {
    match ifaces::ifaces() {
        Ok(interfaces) => {
            for interface in interfaces.into_iter()
                .filter(|i| i.kind == Kind::Ipv4)
                .filter(|i| i.addr.is_some())
                .filter(|i| i.addr.unwrap().ip().to_string() == ip){
                match interface.hop.unwrap() {
                    Broadcast(a) => {
                        match a {
                            SocketAddr::V4(x) => {
                                return format!("{}", x.ip());
                            }
                            SocketAddr::V6(x) => {
                                return format!("{}", x.ip());
                            }
                        }
                    }
                    _ => {}
                }
            }
        },
        Err(_) => println!("Ooops ...")
    };
    "".to_string()
}

fn port_to_streamium(port: u16) -> u16 {
    port.to_be()
}

fn streamium_to_port(port: u16) -> u16 {
    port.to_be()
}

fn ip_to_streamium(ip: &str) -> u32 {
    let mut octets = ip.split(".")
        .map(|x| format!("{:0>3}", x))
        .collect::<Vec<String>>();
    octets.reverse();

    let mut result: u32 = 0;
    for octet in octets.iter().map(|x| x.parse::<u32>().unwrap()) {
        result = result * 256 + octet;
    }

    return result;
}

fn get_port_from_xml(request_data: String) -> Result<String, String> {
    let mut reader = Reader::from_str(request_data.as_str());
    reader.trim_text(true);

    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"Port" => {
                        match reader.read_event(&mut buf) {
                            Ok(Event::Text(e)) => {
                                return Result::Ok(e.unescape_and_decode(&reader).unwrap().parse().unwrap());
                            },
                            _ => return Result::Err("Expecting a text in <Port>!".to_string())
                        }
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(_) => return Result::Err("Error wihle parsing XML".to_string()),
            _ => (), // There are several other `Event`s we do not consider here
        }
    }
    Result::Err("Port not found!".to_string())
}
