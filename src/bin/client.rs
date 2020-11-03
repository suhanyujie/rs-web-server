use std::net::TcpStream;
use std::io::{Write, Read};

fn main() {
    println!("hello this is client...");

    client();
}

fn client() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8000")?;
    let mut index = 1;
    loop {
        let content = String::from("hello...he");
        println!("client sending content is: {}", &content);
        stream.write(content.as_bytes())?;
        stream.flush()?;

        let mut line = [0; 512];
        let result = stream.read(&mut line);
        match result {
            Ok(n) => {
                println!("Received cnt is: [{}] bytes, content is: [{}]. ", n, String::from_utf8_lossy(&line));
            },
            _ => {},
        }
        std::thread::sleep(std::time::Duration::from_secs(3))
    }
    
    Ok(())
}
