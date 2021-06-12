use super::{
    // structs
    ClientServerChannel,
    MessengerActivity,
    Package,
    // enums
    ClientThreadMessenger,
    ActivityResult,
    ProtocolContants,
};

use std::{
    io::{Read,Write},
    net::TcpStream,
    mem::MaybeUninit,
};

pub struct Client{
    pub thread_id:usize,
    pub messenger:ClientThreadMessenger,
    pub channel:ClientServerChannel,
    pub package:Package,

    pub signed_in:bool,
    pub name:String,
}

impl Client{
    pub fn new(thread_id:usize,messenger:ClientThreadMessenger)->Client{
        Self{
            thread_id,
            messenger,
            channel:unsafe{MaybeUninit::uninit().assume_init()},
            package:Package::new(),
            signed_in:false,
            name:String::with_capacity(255),
        }
    }

    /// Установка нового клиента.
    pub fn set_client(&mut self,socket:TcpStream){
        self.channel=ClientServerChannel::new(socket);
    }

    pub fn handle(&mut self){
        let mut task;

        loop{
            task=[0u8];

            if let Err(e)=self.channel.socket.read_exact(&mut task){
                if let std::io::ErrorKind::TimedOut=e.kind(){
                    continue
                }
                else{
                    break
                }
            }

            match ProtocolContants::new(task[0]){
                ProtocolContants::ActivityAuthorization=>{
                    //println!("Sign in");
                        // Получение имени (255 знаков - максимум)
                    if self.channel.read_string(&mut self.name).is_err(){
                        break
                    }

                    //println!("{}",self.name);

                    // Проверка имени
                    if self.name.is_empty(){
                        break
                    }

                    // Регистрация в системе обмена сообщениями
                    self.signed_in=self.messenger.sign_in(self.thread_id,&self.name);

                    //println!("Signed in - {}",self.signed_in);

                    // Отправка результата
                    let result=ProtocolContants::ResultSignInErr as u8-self.signed_in as u8;
                    if self.channel.socket.write_all(&[result]).is_err(){
                        break
                    }

                    //println!("Signed in");
                }

                ProtocolContants::ActivityMessenger=>{
                    //println!("Messenger");
                    if self.signed_in{
                        //println!("Signed");
                        // Передача управления активности
                        let mut messenger_activity=MessengerActivity::new(self);
                        let result=messenger_activity.run();
                        //println!("Messenger closed with {:?}",result);
                        match result{
                            ActivityResult::Closed=>continue,
                            ActivityResult::Disconnected=>break,
                        }
                    }
                    else{
                        break
                    }
                }
                _=>break,
            }
        }

        //println!("Closed client");

        // Удаление пользователя из списка зарегистрированных
        // и очистка очереди сообщений
        if self.signed_in{
            self.messenger.unsign(&self.name);
            self.messenger.clear();
            self.name.clear();
            self.signed_in=false;
        }
    }
}