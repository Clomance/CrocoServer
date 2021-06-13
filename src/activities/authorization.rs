use crate::{
    // structs
    Client,
    // enums
    ActivityResult,
    ProtocolContants,
};

use std::{
    io::{
        Read,
        ErrorKind
    },
};

pub struct AuthorizationActivity<'c>{
    client:&'c mut Client
}

impl<'c> AuthorizationActivity<'c>{
    pub fn new(client:&'c mut Client)->AuthorizationActivity<'c>{
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
                    ProtocolContants::ActivityAuthorization=>{
                        activity_result=ActivityResult::Closed;
                        break
                    }

                    ProtocolContants::TaskSimpleSignIn=>{
                        self.client.server_log.write("Sign in");
                        // Получение имени (255 знаков - максимум)
                        if self.client.channel.read_string(&mut self.client.name).is_err(){
                            break
                        }

                        self.client.server_log.write(&format!("{}",self.client.name));

                        // Проверка имени
                        if self.client.name.is_empty(){
                            break
                        }

                        // Регистрация в системе обмена сообщениями
                        self.client.signed_in=self.client.thread_messenger.sign_in(self.client.thread_id,&self.client.name);

                        self.client.server_log.write(&format!("Signed in - {}",self.client.signed_in));

                        // Отправка результата
                        let result=ProtocolContants::ResultSignInErr as u8-self.client.signed_in as u8;
                        if self.client.channel.send_byte(result).is_err(){
                            break
                        }

                        self.client.server_log.write("Signed in");
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