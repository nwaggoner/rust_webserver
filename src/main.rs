/*  Goes in README.md
 *  Network Stuff
 * PHY      - packets on wire: header w/ address, payload
 *            (eth mac 0a:1b:2c:3d:4e:5f)
 * IPv4     - packets, headers, payload; wrapped in PHY
 *            (IPv4 addr 1.217.3.4)
 *            "best effort" aka it tries it's best to deliver information
 * TCP      - bidi streams, ports, split and wrapped in IPv4
 *            (16-bit port 3000)
 * HTPP     - text packets, headers and body, wrapped by TCP
 *            (textual URL http://1.217.3.4/)
 * HTML     - "special" text in HTTP body
 * TLS      - encryption for TCP streams, protects HTTP
 *            (used for "https")
 * DNS      - map names to IPv4 addresses
 *            (example.org -> 1.217.3.4)
 *
 */

use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use std::io::{BufRead, BufReader, Read, Write};
/*
struct Request {
    verb: String,
    path: String,
    headers: Vec<(String, String)>,
}
fn read_line<R: Read>(reader: &mut BufReader<R>) -> String {
    let mut buf = String::new();
    reader.read_line(&mut
}
*/

/*
pub enum Option<T> {
    None,
    Some(T),
}

pub use Option::*;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub use Result::*;
*/

//let m: u8 = [1, 3, 0].iter().max().unwrap();              //.max() - remember the maximum value from the iterator. 
                                                            //.unwrap() - prevents .max() from causing error or panicking.



fn main() {
    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr = SocketAddrV4::new(localhost, 3000);
    let tcp_listener = TcpListener::bind(socket_addr).unwrap();
    loop {
        let (mut tcp_stream, addr) = tcp_listener.accept().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1000));
        eprintln!("connection from {}", addr);               //tcp_strem is implicitly closed if not used. Used below.
        write!(tcp_stream, "HTTP/1.0 200 OK\r\n\r\n").unwrap();
        write!(tcp_stream, "<html><body><em>Hello world!</em></body></html>\r\n").unwrap();
        tcp_stream.flush().unwrap();
    }
}










