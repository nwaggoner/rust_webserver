use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
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


fn discard_request(tcp_stream: &mut TcpStream) {
    let mut reader = BufReader::new(tcp_stream);
    let mut line = String::new();
    loop {
        reader.read_line(&mut line).unwrap();
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            return;
        }
        eprintln!("{}", trimmed);
        line.clear();
    }
}


fn main() {
    let localhost = Ipv4Addr::new(127, 0, 0, 1);
    let socket_addr = SocketAddrV4::new(localhost, 3000);
    eprintln!("Server starts: {}", socket_addr);
    let tcp_listener = TcpListener::bind(socket_addr).unwrap();
    loop {
        let (mut tcp_stream, addr) = tcp_listener.accept().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1000));
        eprintln!("connection from {}", addr);               //tcp_strem is implicitly closed if not used. Used below.
        discard_request(&mut tcp_stream);
        write!(tcp_stream, "HTTP/1.0 200 OK\r\n\r\n").unwrap();
        write!(tcp_stream, "<html><body><em>Hello world!</em></body></html>\r\n").unwrap();
        tcp_stream.flush().unwrap();
    }
}










