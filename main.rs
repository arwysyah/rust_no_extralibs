#![allow(unused)]
use std::net::{TcpListener, TcpStream};
use std::io::{Read,Write};
use std::result;
use std::fmt;
use std::thread;
use std::sync::mpsc::{Receiver,Sender,channel};
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
fn server(_messages: Receiver<Message>)->Result <()>{
todo!();
} 


fn client_spawn (mut stream :TcpStream,message:Sender<Message>)-> Result<()> {
    // message.send(Message::ClientConnected).map(|err|{
    //     eprintln!("ERROR: could not send message to server thread due to : {err}")
    // })?;
    let _ = writeln!(stream, "hello its ok").map_err(|err|{
    eprintln!("its seems there is something wrong {err}");
    });

    loop {
        unimplemented!();
    }
    todo!();
}



enum Message {
    ClientConnected,
    ClientDisconnected,
    NewMessage,
}

fn main() -> Result<()> {
    let address = "0.0.0.0:8000";
    let listener = TcpListener::bind(address).map_err(|err|{
        eprintln!("Error connect to {address} due to :{err}", err=Sensitive(err));
    })?;

    println!("INFO: Listening to {}",Sensitive(address));
    let (message_sender,message_receiver) = channel();
    thread::spawn(|| server(message_receiver));
    for stream in listener.incoming() {
            match stream {
         
        Ok(stream) => {
 let message_sender = message_sender.clone(); 
            thread::spawn( || client_spawn(stream,message_sender));
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


