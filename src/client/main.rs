use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap(); // bind to any available port;

    socket.connect("3.141.190.248:2700").unwrap();

    let message = b"Test message";

    loop {
        socket.send(message).unwrap();

        let mut buffer = [0; 1024];

        let amount = socket.recv(&mut buffer).unwrap();

        println!("Received: {}", String::from_utf8_lossy(&buffer[..amount]));
    }
    


}
