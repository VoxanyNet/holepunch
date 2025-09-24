use std::{net::{SocketAddr, UdpSocket}, thread::sleep, time::Duration};

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:2700").unwrap();

    let mut buffer = [0u8; 1028];

    let mut clients: Vec<SocketAddr> = Vec::new();

    loop {
        let (amount, src) = socket.recv_from(&mut buffer).unwrap();


        dbg!(&src);

        if !clients.contains(&src) {
            clients.push(src);
        }
        
        socket.send_to(src.to_string().as_bytes(), &src).unwrap();

        
    }
}