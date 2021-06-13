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

pub struct ControlActivity<'c>{
    client:&'c mut Client,
}

impl<'c> ControlActivity<'c>{
    pub fn new(client:&'c mut Client)->ControlActivity<'c>{
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
                    ProtocolContants::ActivityControl=>{
                        activity_result=ActivityResult::Closed;
                        break
                    }

                    ProtocolContants::TaskCheckLog=>{
                        let log=self.client.server_log.log.lock().unwrap();
                        let lines=log.lines();
                        let [part1,part2]=log.read();

                        self.client.package.clear();
                        self.client.package.write(&lines.to_be_bytes());
                        self.client.package.write(part1);
                        self.client.package.write(part2);

                        if self.client.channel.send_package(&self.client.package).is_err(){
                            break
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