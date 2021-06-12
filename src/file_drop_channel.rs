use super::{
    package_size,
};

use std::{
    sync::{
        RwLock,
        RwLockReadGuard
    },
};

pub fn file_drop_channel_channel()->(Writer,Reader){
    let mut writer=Writer{
        channel:Box::new(FileDropChannel::new())
    };

    let reader=Reader{
        ptr:writer.channel_ptr(),
    };

    (writer,reader)
}

pub struct Writer{
    channel:Box<FileDropChannel>
}

impl Writer{
    pub fn write(&mut self,bytes:&[u8]){
        self.channel.as_mut().write(bytes)
    }

    pub fn channel_ptr(&mut self)->*mut FileDropChannel{
        self.channel.as_mut() as *mut FileDropChannel
    }
}

pub struct Reader{
    ptr:*mut FileDropChannel,
}

impl Reader{
    pub fn read<'a>(&self)->RwLockReadGuard<'a,Vec<u8>>{
        unsafe{
            let channel=&mut *self.ptr;
            channel.read()
        }
    }
}

#[derive(PartialEq)]
enum Buffer{
    First,
    Second,
}

pub struct FileDropChannel{
    read_buffer:Buffer,
    write_buffer:Buffer,
    buffer1:RwLock<Vec<u8>>,
    buffer2:RwLock<Vec<u8>>,
}

impl FileDropChannel{
    pub fn new()->FileDropChannel{
        Self{
            read_buffer:Buffer::First,
            write_buffer:Buffer::First,
            buffer1:RwLock::new(Vec::with_capacity(package_size)),
            buffer2:RwLock::new(Vec::with_capacity(package_size)),
        }
    }

    pub fn read<'a>(&'a mut self)->RwLockReadGuard<'a,Vec<u8>>{
        if self.read_buffer==Buffer::Second{
            self.read_buffer=Buffer::First;
            self.buffer2.read().unwrap()
        }
        else{
            self.read_buffer=Buffer::Second;
            self.buffer1.read().unwrap()
        }
    }

    pub fn write(&mut self,bytes:&[u8]){
        let mut buffer=if self.write_buffer==Buffer::First{
            self.write_buffer=Buffer::Second;
            self.buffer1.write().unwrap()
        }
        else{
            self.write_buffer=Buffer::First;
            self.buffer2.write().unwrap()
        };

        buffer.clear();
        for &byte in bytes{
            buffer.push(byte);
        }
    }
}