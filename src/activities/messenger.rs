use crate::{
    // structs
    Client,
    // enums
    ClientThreadMessage,
    ActivityResult,
    ProtocolContants,
};

use std::{
    io::{Read,ErrorKind},
};

pub struct MessengerActivity<'c>{
    client:&'c mut Client
}

impl<'c> MessengerActivity<'c>{
    pub fn new(client:&'c mut Client)->MessengerActivity<'c>{
        Self{
            client,
        }
    }

    pub fn run(&mut self)->ActivityResult{
        let mut activity_result=ActivityResult::Disconnected;

        let mut task;

        loop{
            task=[0u8];

            // Ожидание задачи от клиента
            // При превышении лимита ожидания начанает цикл заново
            // (повторяет предыдущие и это действия)

            match self.client.channel.socket.read_exact(&mut task){
                Ok(_)=>match ProtocolContants::new(task[0]){
                    // Выход из активности
                    ProtocolContants::ActivityMessenger=>{
                        activity_result=ActivityResult::Closed;
                        break
                    }

                    ProtocolContants::TaskSendMessage=>{
                        //println!("Send message");
                        let mut user=String::with_capacity(255);
                        let mut text=String::with_capacity(255);
                        // Получение имени (255 знаков - максимум)
                        if self.client.channel.read_string(&mut user).is_err()||
                                self.client.channel.read_string(&mut text).is_err(){
                            break
                        }
                        //println!("Sending");
                        // Отправка сообщения
                        let sended=self.client.messenger.send(&user,ClientThreadMessage::Text{
                            from:self.client.name.clone(),
                            text:text,
                        });

                        let result=ProtocolContants::ResultSendMessageErr as u8-sended as u8;
                        if self.client.channel.send_byte(result).is_err(){
                            break
                        }

                        //println!("Sent message");
                    }

                    ProtocolContants::TaskCheckMessages=>{
                        // Проверка наличия сообщений для данных потока и клиента
                        if let Some(message)=self.client.messenger.receive(){
                            match message{
                                ClientThreadMessage::Text{
                                    from,
                                    text,
                                }=>{
                                    //println!("Got a message");
                                    self.client.package.clear();
                                    self.client.package.write_task(ProtocolContants::MessageText as u8);
                                    self.client.package.write_string(&from);
                                    self.client.package.write_string(&text);
                                    //println!("Sending message");
                                    if self.client.channel.send_package(&self.client.package).is_err(){
                                        //println!("Error");
                                        break
                                    }
                                    //println!("Sent");
                                }
                                _=>{}
                            }
                        }
                        else{
                            if self.client.channel.send_byte(ProtocolContants::MessageNothing as u8).is_err(){
                                break
                            }
                        }
                    }
                    _=>break
                }
                Err(e)=>match e.kind(){
                    ErrorKind::TimedOut=>continue,
                    _=>break
                }
            }
        }

        activity_result
    }
}