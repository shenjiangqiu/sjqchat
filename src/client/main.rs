use std::{
    env::args,
    io::{Read, Write},
    net::TcpStream,
};
fn main() {
    let addr = args()
        .nth(1)
        .unwrap_or_else(|| "thesjq.com:2233".to_string());

    let mut stream = TcpStream::connect(addr).unwrap();
    loop {
        println!("write your text(exit to close the app!):\n");
        let mut stdin = String::new();
        std::io::stdin().read_line(&mut stdin).unwrap();
        let stdin_buffer = stdin.as_bytes();
        let len = stdin_buffer.len() as u32;
        if stdin.starts_with("exit") || stdin.starts_with("quit") {
            let result = 0u32.to_be_bytes();
            stream.write_all(&result).unwrap();
            break;
        }
        stream.write_all(&len.to_be_bytes()).unwrap();
        stream.write_all(stdin_buffer).unwrap();

        println!("waiting for response:\n");
        let mut read_buffer = [0; 1024];
        let mut buffer = 0u32.to_be_bytes();
        stream.read_exact(&mut buffer).unwrap();
        let txt_type = u32::from_be_bytes(buffer);
        if txt_type == 0 {
            println!("received stop signal");
            break;
        }
        let recv_buffer = &mut read_buffer[0..txt_type as usize];
        stream.read_exact(recv_buffer).unwrap();
        println!("the other says: {}\n", String::from_utf8_lossy(recv_buffer));
    }
}
