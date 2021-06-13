use super::Package;

use std::{
    io::{
        Write,
        Read
    },
    net::TcpStream,
};

pub struct ClientServerChannel{
    pub socket:TcpStream,
}

impl ClientServerChannel{
    #[inline(always)]
    pub fn new(socket:TcpStream)->ClientServerChannel{
        Self{
            socket,
        }
    }

    pub fn read_is_empty(&self)->std::io::Result<bool>{
        let mut buf=[0u8];
        match self.socket.peek(&mut buf){
            Ok(bytes)=>if bytes==0{
                Ok(true)
            }
            else{
                Ok(false)
            }
            Err(e)=>Err(e),
        }
    }

    //pub fn peek(&mut self,)

    #[inline(always)]
    pub fn flush(&mut self)->std::io::Result<()>{
        self.socket.flush()
    }
}

/// Чтение.
impl ClientServerChannel{
    /// Читает строку.
    pub fn read_string(&mut self,string:&mut String)->std::io::Result<()>{
        let mut byte=[0u8];
        self.socket.read_exact(&mut byte)?;

        string.clear();

        let len=byte[0] as usize;
        let bytes=unsafe{
            let vec=string.as_mut_vec();
            vec.set_len(len);
            vec
        };

        self.socket.read_exact(&mut bytes[0..len])?;

        Ok(())
    }
}

// Отправка-запись.
impl ClientServerChannel{
    pub fn write_bytes(&mut self,bytes:&[u8])->std::io::Result<()>{
        self.socket.write_all(bytes)
    }

    pub fn write_string(&mut self,string:&str)->std::io::Result<()>{
        let len=string.len() as u8;

        self.socket.write(&[len])?;

        self.socket.write_all(string.as_bytes())
    }

    pub fn send_byte(&mut self,byte:u8)->std::io::Result<()>{
        self.socket.write_all(&[byte])?;
        self.flush()
    }

    pub fn send_package(&mut self,package:&Package)->std::io::Result<()>{
        self.write_bytes(package.read())?;
        self.flush()
    }
}