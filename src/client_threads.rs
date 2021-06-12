use super::{
    client_thread_amount,
    client_thread_stack_size,
    Client,
    ClientThreadMessengerBuilder,
};

use std::{
    net::TcpStream,
    sync::{
        Arc,
        Mutex,
        mpsc::{channel,Sender}
    },
    thread::Builder,
};

pub struct ClientThreads{
    sender:Sender<TcpStream>,
}

impl ClientThreads{
    pub fn start()->ClientThreads{
        // Канал для передачи клиента в свободный поток для обработки
        let (client_sender,client_receiver)=channel::<TcpStream>();
        let client_receiver=Arc::new(Mutex::new(client_receiver));

        let mut client_thread_messanger_builder=ClientThreadMessengerBuilder::new(client_thread_amount);

        let mut thread_id=0;
        while let Some(messanger)=client_thread_messanger_builder.next(){
            let name=format!("thread_{}",thread_id);
            let client_receiver=client_receiver.clone();

            Builder::new().name(name).stack_size(client_thread_stack_size).spawn(move||{
                // Создание структуры для обработки клиента
                let mut client=Client::new(thread_id,messanger);

                loop{
                    // Ожидание клиента
                    let stream=client_receiver.lock().unwrap().recv().unwrap();
                    // Передача клиента
                    client.set_client(stream);
                    // Запуск обработчика
                    let _=client.handle();
                }
            }).unwrap();

            thread_id+=1;
        }

        Self{
            sender:client_sender
        }
    }

    /// Отправка клиента на обработку потокам.
    pub fn handle(&self,socket:TcpStream){
        let _=self.sender.send(socket);
    }
}