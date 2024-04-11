use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    use std::io::prelude::*;
    use std::net::{TcpStream};
    use ssh2::Session;
    let host = String::from("0.0.0.0");
    println!("connect to {}", host);
    // Connect to the local SSH server
    
    let tcp = TcpStream::connect(host).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_agent("user").unwrap();
    
    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    let _ = channel.wait_close();
    println!("{}", channel.exit_status().unwrap());
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);

    // Print the elapsed time in seconds and milliseconds
    println!("Elapsed time: {}.{:03} seconds", elapsed_time.as_secs(), elapsed_time.subsec_millis());
    }


