use rcgen::generate_simple_self_signed;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::net::UdpSocket;
use std::thread;
mod discover;

fn main() {
    let mut config_dir = dirs::config_dir().expect("Unable to find config directory.");
    config_dir.push("connectd");
    create_dir_all(config_dir.as_path()).expect("Can't create config directory.");

    let mut private_key_path = config_dir.as_path().to_path_buf();
    private_key_path.push("private_key.dem");
    let mut public_key_path = config_dir.as_path().to_path_buf();
    public_key_path.push("public_key.dem");

    let mut private_key = vec![];
    let mut public_key = vec![];
    if !private_key_path.is_file() || !public_key_path.is_file() {
        // Generate private and public keys
        let cert = generate_simple_self_signed(vec![]).unwrap();
        private_key = cert.serialize_private_key_der();
        public_key = cert.serialize_der().unwrap();
        let mut private_key_file =
            File::create(private_key_path).expect("Can't create private key file.");
        let mut public_key_file =
            File::create(public_key_path).expect("Can't create public key file.");
        private_key_file
            .write_all(&private_key)
            .expect("Can't write private key file.");
        public_key_file
            .write_all(&public_key)
            .expect("Can't write public key file.");
    } else {
        // Read existing private and public keys
        let mut private_key_file =
            File::open(private_key_path).expect("Can't open private key file.");
        let mut public_key_file = File::open(public_key_path).expect("Can't open public key file.");
        private_key_file
            .read_to_end(&mut private_key)
            .expect("Can't read private key file.");
        public_key_file
            .read_to_end(&mut public_key)
            .expect("Can't read public key file.");
    }

    let broadcast_socket = UdpSocket::bind(discover::ADDR).unwrap();
    let broadcast_socket2 = broadcast_socket.try_clone().unwrap();
    let hostname = match hostname::get() {
        Ok(x) => x.to_str().unwrap_or("anonymous").to_owned(),
        Err(_) => "anonymous".to_string(),
    };
    let t = thread::spawn(move || discover::discoverable(broadcast_socket2, &hostname));
    let discovered = discover::discover(broadcast_socket).unwrap();
    println!("discovered: {:?}", discovered);
    t.join().unwrap().unwrap();
}
