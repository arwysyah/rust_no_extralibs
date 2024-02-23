#![allow(unused)]
use std::net::{TcpListener, TcpStream};
use std::io::{Read,Write};
use std::result;
use std::fmt;
type Result<T> = result::Result<(),(T)>;

struct Sensitive <T> {
    inner: T
}
impl <T> Sensitive <T> {
    fn new(inner:T)->Self {
        Self{inner}
    }
}



const SAFE_MODE: bool = true;

impl <T: fmt::Display> fmt::Display for Sensitive<T> {

    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result {
        if SAFE_MODE {
            writeln!(f,"[REDACTED]")
        }else{
            writeln!(f,"{inner}",inner = self.inner)
        }
    }

}
fn handle_client(mut stream: TcpStream) {
        
    let mut buffer = [0;1024];
    let bytes_read = stream.read(&mut buffer).expect("Failed to read from stream");

    if bytes_read > 0{
        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received {}",message);
    }else{
        println!("no data received");
    }
    // print!();
    // ...
}



fn main() -> Result<()> {
    let address = "127.0.0.1:8000";
    let listener = TcpListener::bind(address).map_err(|err|{
        eprintln!("Error connect to {} due to :{}", Sensitive::new(address),Sensitive::new(err));
    })?;

    println!("INFO: Listening to {}",Sensitive::new(address));

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
             let _ =   writeln!(stream, "hello its ok").map_err(|err|{
                    eprintln!("its seems there is something wrong {err}");
                });
                println!("OK ");
            },
            Err(err) =>{
                eprintln!("Error: could not accept {err}")
            } 
        }
    }
    // accept connections and process them serially
    // for stream in listener.incoming() {
    //     handle_client(stream?);
//}
Ok(())
}
