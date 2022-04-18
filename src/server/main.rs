use std::env::args;
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let addr = args().nth(1).unwrap_or_else(|| ":::2233".to_string());

    let listener = TcpListener::bind(addr).unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    while let Ok((mut stream, addr)) = listener.accept() {
        println!("Accepted connection from {}", addr);
        // read the header of usize
        let mut read_buffer= [0;1024];
        loop{
            println!("waiting for text from another...:\n");
            // first read and then write
            let mut buffer = 0usize.to_be_bytes();
            stream.read_exact(&mut buffer).unwrap();
            let txt_type = usize::from_be_bytes(buffer);
            if txt_type == 0 {
                println!("received stop signal");
                break;
            }
            let recv_buffer=& mut read_buffer[0..txt_type as usize];
            stream.read_exact(recv_buffer).unwrap();

            println!("the other says: {}\n",String::from_utf8_lossy(recv_buffer));

            //read awnser from stdin
            println!("write your response:\n");
            let mut stdin = String::new();
            std::io::stdin().read_line(&mut stdin).unwrap();
            let stdin_buffer = stdin.as_bytes();
            let len=stdin_buffer.len();
            stream.write_all(&len.to_be_bytes()).unwrap();
            stream.write_all(stdin_buffer).unwrap();
        }

        
        //close stream
        // stream.shutdown(std::net::Shutdown::Both).unwrap();
    }
}
