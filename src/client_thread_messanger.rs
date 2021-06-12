use super::Reader;

use std::{
    sync::{
        Arc,
        Mutex,
        RwLock,
        mpsc::{
            channel,
            Sender,
            Receiver,
        }
    },
    collections::HashMap,
};

/// Потоковые сообщения.
pub enum ClientThreadMessage{
    Text{
        /// От кого сообщение.
        from:String,
        /// Текст сообщения.
        text:String,
    },

    /// Предложение отправки файла.
    FileDropOffer{
        /// От кого файл.
        from:String,
        /// Структура для чтения файла.
        reader:Reader,
    },
    FileDropAccepted,
    FileDropDenied,
}

unsafe impl Send for ClientThreadMessage{}
unsafe impl Sync for ClientThreadMessage{}

/// Структура для построения системы обмена сообщениями.
pub struct ClientThreadMessengerBuilder{
    current_client:usize,
    clients_limit:usize,
    message_senders:Arc<Vec<Mutex<Sender<ClientThreadMessage>>>>,
    message_senders_ptr:*mut Vec<Mutex<Sender<ClientThreadMessage>>>,
    signed_clients:Arc<RwLock<HashMap<String,usize>>>,
}

impl ClientThreadMessengerBuilder{
    /// Создаёт основу для построения системы обмена сообщениями.
    pub fn new(clients_limit:usize)->ClientThreadMessengerBuilder{
        let mut message_senders=Arc::new(Vec::with_capacity(clients_limit));

        let message_senders_ptr=Arc::get_mut(&mut message_senders).unwrap() as *mut _;

        Self{
            current_client:0usize,
            clients_limit,
            message_senders,
            message_senders_ptr,
            signed_clients:Arc::new(RwLock::new(HashMap::<String,usize>::with_capacity(clients_limit)))
        }
    }

    /// Строит новый канал для обмена сообщениями.
    pub fn next(&mut self)->Option<ClientThreadMessenger>{
        if self.current_client<self.clients_limit{
            self.current_client+=1;
            // Канал для передачи сообщений
            let (sender,receiver)=channel();
            // Добавление отправителя в массив для общего доступа
            unsafe{&mut *self.message_senders_ptr}.push(Mutex::new(sender));

            Some(
                ClientThreadMessenger{
                    message_senders:self.message_senders.clone(),
                    signed_clients:self.signed_clients.clone(),
                    message_receiver:receiver,
                }
            )
        }
        else{
            None
        }
    }
}

/// Система обмена сообщениями между потоками.
pub struct ClientThreadMessenger{
    /// Массив каналов для отправки сообщения от одного клиента другому.
    /// Индекс канала соответствует номеру потока.
    message_senders:Arc<Vec<Mutex<Sender<ClientThreadMessage>>>>,
    /// Зарегистрированные клиенты.
    signed_clients:Arc<RwLock<HashMap<String,usize>>>,
    /// Собственный получатель сообщений.
    message_receiver:Receiver<ClientThreadMessage>,
}

impl ClientThreadMessenger{
    /// Зарегистрирует клиентов.
    pub fn sign_in(&self,thread_id:usize,name:&String)->bool{
        let mut signed_clients=self.signed_clients.write().unwrap();

        if let Some(_)=signed_clients.get(name){
            false
        }
        else{
            signed_clients.insert(name.clone(),thread_id);
            true
        }
    }

    /// Убирает клиентов клиентов из списка зарегистрированных.
    pub fn unsign(&self,name:&String){
        self.signed_clients.write().unwrap().remove(name);
    }

    /// Отправляет сообщение клиенту.
    /// Если нет такого клиента, возвращает false.
    pub fn send(&self,name:&String,message:ClientThreadMessage)->bool{
        if let Some(&thread_id)=self.signed_clients.read().unwrap().get(name){
            self.message_senders.get(thread_id).unwrap().lock().unwrap().send(message).unwrap();
            true
        }
        else{
            false
        }
    }

    /// Возвращает отправленные этому клиенту сообщения.
    pub fn receive(&self)->Option<ClientThreadMessage>{
        if let Ok(message)=self.message_receiver.try_recv(){
            Some(message)
        }
        else{
            None
        }
    }

    /// Очищает очередь сообщений, перебирая все.
    pub fn clear(&self){
        let mut iter=self.message_receiver.iter();
        while let Some(_)=iter.next(){}
    }
}