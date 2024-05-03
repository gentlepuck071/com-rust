use std::net::TcpStream;
use std::io::prelude::*;
use ssh2::Session;
use std::io;
use std::time::Instant;

#[derive(Debug)]
pub struct SSHClient {
    host: String,
    username: String,
    password: String,
    port: u16,
}

impl SSHClient {
    pub fn new(host: String, username: String, password: String, port: u16) -> SSHClient {
        SSHClient {
            host: host.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            port,
        }
    }

    pub fn execute_command(&self, command:String) -> Result<String, ssh2::Error> {
        let _tcp = TcpStream::connect(format!("{}:{}", self.host, self.port));
        let mut session = Session::new()?;
        session.handshake()?;

        session.userauth_password(&self.username, &self.password)?;

        let mut channel = session.channel_session()?;
        channel.exec(&command)?;

        let mut s = String::new();
        let _ =channel.read_to_string(&mut s);

        Ok(s)
    }
}
fn main() {
    let mut host_num = String::new();
    let mut username = String::new();
    let mut password = String::new();
    let mut command = String::new();
    let executing_command:String;
    io::stdin()
        .read_line(&mut host_num)        
        .expect("Failed to read line");
    io::stdin()
        .read_line(&mut username)        
        .expect("Failed to read line");
    io::stdin()
        .read_line(&mut password)        
        .expect("Failed to read line");
    io::stdin()
        .read_line(&mut command)        
        .expect("Failed to read line");
    let start_time = Instant::now();

    let ssh_client = SSHClient{
        host:host_num,
        username:username,
        password:password,
        port:22,
    };

    println!("{:?}", &ssh_client);
    if ssh_client.username != "root" {
        executing_command = format!("sudo -S -p '' {}", command);
        let _ = ssh_client.execute_command(executing_command);
    }
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);

    // Print the elapsed time in seconds and milliseconds
    println!("Elapsed time: {}.{:03} seconds for SSH_Client", elapsed_time.as_secs(), elapsed_time.subsec_millis());
    
}
