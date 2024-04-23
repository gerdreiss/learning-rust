use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    // let stream = listener.accept();
    // println!("Connection established: {:?}", stream);

    // loop {
    //     match listener.accept() {
    //         Ok((stream, addr)) => {
    //             println!("Connection established: {:?}, address: {:?}", stream, addr);
    //         }
    //         Err(e) => {
    //             println!("Failed to establish connection: {:?}", e);
    //             break;
    //         }
    //     }
    // }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => println!("Failed to establish connection: {:?}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let http_request = BufReader::new(&mut stream)
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    println!("Request: {:?}", http_request.join("\n"));

    let response = "HTTP/1.1 200 OK\r\n\r\nOK";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
