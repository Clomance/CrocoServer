#![allow(non_snake_case,non_upper_case_globals,dead_code,invalid_value)]

mod sync_raw_ptr;
//use sync_raw_ptr::SyncRawPtr;

mod protocol_contants;
use protocol_contants::ProtocolContants;

mod package;
use package::Package;

mod client;
use client::Client;

mod client_threads;
use client_threads::ClientThreads;

mod client_server_channel;
use client_server_channel::ClientServerChannel;

mod file_drop_channel;
use file_drop_channel::{
    //file_drop_channel_channel,
    Reader,
    //FileDropChannel,
};

mod client_thread_messanger;
use client_thread_messanger::{
    ClientThreadMessage,
    ClientThreadMessenger,
    ClientThreadMessengerBuilder
};

mod activities;
use activities::{
    ActivityResult,
    MessengerActivity,
};

use std::{
    net::{
        Ipv4Addr,
        IpAddr,
        SocketAddr,
        TcpListener,
    },
    time::Duration,
};

// Время ожидания получения/отправки данных
const default_client_rw_timeout:Duration=Duration::from_micros(2500);

// Количество потоков для обработки клиентов
pub const client_thread_amount:usize=2;

// Размер стека для клиентских потоков
pub const client_thread_stack_size:usize=2*1024*1024; //bytes

// Пакет
// u16 - размер, 0 - стандартный размер (4096)
// [u8] - данные
const package_size:usize=4096;

fn main(){
    let ip=IpAddr::V4(Ipv4Addr::new(192,168,0,101));
    let address=SocketAddr::new(ip,8080);

    // Регистрация сервера
    //println!("Binding");
    let server_socket=TcpListener::bind(address).unwrap();

    // Создание клиентских потоков
    //println!("Creating threads");
    let client_threads=ClientThreads::start();

    loop{
        // Ожидание подключения
        match server_socket.accept(){
            Ok((client_socket,_address))=>{
                // Установка времени ожидания для отправки и принятия данных от клиента
                if client_socket.set_read_timeout(Some(default_client_rw_timeout)).is_ok() &&
                        client_socket.set_write_timeout(Some(default_client_rw_timeout)).is_ok() &&
                        client_socket.set_nodelay(true).is_ok()
                {
                    //println!("Got connection");
                    client_threads.handle(client_socket);
                }
            },
            Err(_e)=>{

            }
        }
    }
}