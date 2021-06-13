use super::{
    // structs
    ServerLogRef,
    ClientServerChannel,
    ControlActivity,
    MessengerActivity,
    AuthorizationActivity,
    Package,
    // enums
    ClientThreadMessenger,
    ActivityResult,
    ProtocolContants,
};

use std::{
    io::Read,
    net::TcpStream,
    mem::MaybeUninit,
};

pub struct Client{
    pub thread_id:usize,
    pub server_log:ServerLogRef,
    pub thread_messenger:ClientThreadMessenger,
    pub channel:ClientServerChannel,
    pub package:Package,

    pub signed_in:bool,
    pub name:String,
}

impl Client{
    pub fn new(thread_id:usize,server_log:ServerLogRef,thread_messenger:ClientThreadMessenger)->Client{
        Self{
            thread_id,
            server_log,
            thread_messenger,
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
                    let mut authorization_activity=AuthorizationActivity::new(self);
                    let result=authorization_activity.run();
                    self.server_log.write(&format!("Authorization closed with {:?}",result));
                    match result{
                        ActivityResult::Closed=>continue,
                        ActivityResult::Disconnected=>break,
                    }
                }

                ProtocolContants::ActivityControl=>{
                    let mut control_activity=ControlActivity::new(self);
                    let result=control_activity.run();
                    self.server_log.write(&format!("Control closed with {:?}",result));
                    match result{
                        ActivityResult::Closed=>continue,
                        ActivityResult::Disconnected=>break,
                    }
                }

                ProtocolContants::ActivityMessenger=>{
                    self.server_log.write("Messenger");
                    if self.signed_in{
                        self.server_log.write("Signed");
                        // Передача управления активности
                        let mut messenger_activity=MessengerActivity::new(self);
                        let result=messenger_activity.run();
                        self.server_log.write(&format!("Messenger closed with {:?}",result));
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

        self.server_log.write("Closed client");

        // Удаление пользователя из списка зарегистрированных
        // и очистка очереди сообщений
        if self.signed_in{
            self.thread_messenger.unsign(&self.name);
            self.thread_messenger.clear();
            self.name.clear();
            self.signed_in=false;
        }
    }
}