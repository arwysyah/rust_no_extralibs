#![allow(unused)]
use std::net::{TcpListener, TcpStream};
use std::io::{Read,Write};
use std::result;
use std::fmt;
use std::thread;
type Result<T> = result::Result<(),(T)>;

// struct Sensitive <T> {
//     inner: T
// }

struct Sensitive <T>(T); //tuple struct
// impl <T> Sensitive <T> {
//     fn new(inner:T)->Self {
//         Self{inner}
//     }
// }
//


const SAFE_MODE: bool = false;

impl <T: fmt::Display> fmt::Display for Sensitive<T> {

    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result {
       let Self(inner) = self;
        
        if SAFE_MODE {
            writeln!(f,"[REDACTED]")
        }else{
            writeln!(f,"{inner}")
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


fn client_spawn (mut stream :TcpStream)-> Result<()> {

   let _ =   writeln!(stream, "hello its ok").map_err(|err|{
                    eprintln!("its seems there is something wrong {err}");
                });
     todo!();

}

fn main() -> Result<()> {
    let address = "0.0.0.0:8000";
    let listener = TcpListener::bind(address).map_err(|err|{
        eprintln!("Error connect to {address} due to :{err}", err=Sensitive(err));
    })?;

    println!("INFO: Listening to {}",Sensitive(address));

    for stream in listener.incoming() {
            match stream {
           
        Ok(stream) => {
            thread::spawn( || client_spawn(stream));
        }
        Err(err)=>{
                eprintln!("ERROR: could not accept connection: {err}")
            }
    }
    }

    // accept connections and process them serially
    // for stream in listener.incoming() {
    //     handle_client(stream?);
//}
Ok(())
}


