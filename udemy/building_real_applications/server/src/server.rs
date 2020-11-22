use std::io;
use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(&self) {
        println!("Listening on {}", &self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            listener
                .accept()
                .and_then(|(mut stream, addr)| self.process_request(addr, &mut stream));

            // match listener.accept() {
            //     Err(err) => eprintln!("Failed to establish connection: {}", err),
            //     Ok((mut stream, addr)) => process_request(addr, &mut stream)
            // }
        }
    }

    fn process_request(&self, addr: SocketAddr, stream: &mut TcpStream) -> io::Result<()> {
        println!("Request received from {}", addr);

        let mut buf = Vec::new();

        stream
            .read_to_end(&mut buf)
            .and_then(|_| {
                let req = String::from_utf8_lossy(buf.as_mut_slice());
                println!("Request received: {}", req);
                Ok(())
            })
    }
}
