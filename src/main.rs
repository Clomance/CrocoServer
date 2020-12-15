#![allow(non_snake_case)]

use std::net::{Ipv4Addr,IpAddr,SocketAddr,TcpListener,
    //TcpStream
};
// use std::fs::OpenOptions;
// use std::io::{stdin,Read,Write};

// use std::path::Path;

// use std::time::Duration;

// Пакет
// u16 - размер, 0 - стандартный размер (4096)
// [u8] - данные

// const package_size:usize=4096;

// const sign_in:i8=4;

fn main(){
    let ip=IpAddr::V4(Ipv4Addr::new(192,168,0,100));
    let address=SocketAddr::new(ip  ,8080);

    let server_socket=TcpListener::bind(address).unwrap();

    
}
