use std::sync::{
    Arc,
    Mutex,
};

/// Должна быть больше 256
const server_log_size:usize=1024;

/// Журнал событий сервера.
/// 
/// Максимальная длина одной строки - 255 символов, кодировка UTF-8.
/// 
/// Журнал имеет определённый размер.
/// После привышения размера начинается запись поверх старых данных (режим переписывания).
pub struct ServerLog{
    /// Режим переписывания.
    rewrite:bool,
    /// Начало журнала.
    start:usize,
    /// Конец журнала.
    end:usize,
    /// Количество строк.
    lines:usize,
    /// Данные.
    data:Vec<u8>,
}

impl ServerLog{
    pub fn new()->ServerLog{
        let mut data=Vec::with_capacity(server_log_size);
        unsafe{
            data.set_len(server_log_size);
        }

        Self{
            rewrite:false,
            start:0usize,
            end:0usize,
            lines:0usize,
            data,
        }
    }

    pub fn lines(&self)->u16{
        self.lines as u16
    }

    pub fn write_byte(&mut self,byte:u8){
        self.data[self.end]=byte;
        self.end+=1;
    }

    pub fn write(&mut self,line:&str){
        self.lines+=1;

        let len=line.len() as u8;
        let mut bytes=line.as_bytes().iter();

        // вместимость до конца массива
        let mut capacity=server_log_size-self.end;

        if self.rewrite{
            // вся вместимость
            let exact_capacity=if self.start<self.end{
                server_log_size+self.start-self.end
            }
            else{
                server_log_size-self.start+self.end
            };

            if exact_capacity<len as usize{
                self.clear_characters(len as usize-exact_capacity);
            }
        }

        if capacity>line.len(){
            self.write_byte(len);
            for &byte in bytes{
                self.write_byte(byte);
            }
        }
        else if capacity>0{
            self.write_byte(len);

            capacity-=1;

            // добавляем в конец по-максимуму
            for _ in 0..capacity{
                self.write_byte(*bytes.next().unwrap());
            }

            self.end=0;

            if !self.rewrite{
                self.rewrite=true;
                self.clear_characters(len as usize-capacity);
            }

            // Остальное дописываем в начало
            for &byte in bytes{
                self.write_byte(byte);
            }
        }
        else{
            self.end=0;

            if !self.rewrite{
                self.rewrite=true;
                self.clear_characters(len as usize);
            }

            self.write_byte(len);
            for &byte in bytes{
                self.write_byte(byte);
            }
        }
    }

    pub fn read(&self)->[&[u8];2]{
        if self.rewrite{
            [
                &self.data[self.start..self.end],
                &[]
            ]
        }
        else{
            if self.start<self.end{
                [
                    &self.data[self.end..server_log_size],
                    &self.data[0..self.start]
                ]
            }
            else{
                [
                    &self.data[self.start..server_log_size],
                    &self.data[0..self.end]
                ]
            }
        }
    }

    pub fn clear_characters(&mut self,mut characters:usize){
        while characters>0{
            let len=self.data[self.start] as usize+1;
            self.lines-=1;
            self.start+=len;

            if self.start>=server_log_size{
                self.start-=server_log_size;
            }

            characters-=len;
        }
    }

    pub fn clear(&mut self){
        self.rewrite=false;
        self.start=0usize;
        self.end=0usize;
    }
}

#[derive(Clone)]
pub struct ServerLogRef{
    pub log:Arc<Mutex<ServerLog>>,
}

impl ServerLogRef{
    pub fn new(log:ServerLog)->ServerLogRef{
        Self{
            log:Arc::new(Mutex::new(log))
        }
    }

    pub fn write(&mut self,line:&str){
        self.log.lock().unwrap().write(line);
    }
}