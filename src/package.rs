use super::package_size;

/// Пакет для передачи данных.
pub struct Package{

    data:Vec<u8>,
}

impl Package{
    pub fn new()->Package{
        Self{
            data:Vec::with_capacity(package_size),
        }
    }

    pub fn write(&mut self,bytes:&[u8]){
        for &b in bytes{
            self.data.push(b)
        }
    }

    pub fn write_task(&mut self,task:u8){
        self.data.push(task)
    }

    pub fn write_string(&mut self,string:&str){
        let len=string.len() as u8;
        self.write(&[len]);
        self.write(string.as_bytes())
    }

    // Подготавливает пакет к отправке и возвращает ссылку на данные.
    pub fn read(&self)->&[u8]{
        self.data.as_slice()
    }

    pub fn clear(&mut self){
        self.data.clear();
    }
}

impl Package{
    // /// Подготавливает пакет к отправке.
    // pub fn wrap(&mut self){
    //     let size=self.data.len() as u16;
        
    // }

    // pub fn clear(&mut self){
    //     unsafe{
    //         self.data.set_len(2);
    //     }
    // }
}