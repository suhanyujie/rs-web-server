use crate::interface::imessage::IMessage;

pub struct Message {
    pub msg_id: u32,
    pub msg_len: u32,
    pub data: Vec<u8>,
}

impl IMessage for Message {
    fn set_msg_id(&mut self, id: u32) {
        self.msg_id = id;
    }
    fn get_msg_id(&self) -> u32 {
        return self.msg_id;
    }
    fn set_msg_len(&mut self, len: u32) {
        self.msg_len = len;
    }
    fn get_msg_len(&self) -> u32 {
        return self.msg_len;
    }
    fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }
    fn get_data(&self) -> &[u8] {
        return &self.data;
    }
}

impl Message {
    pub fn new(id: u32, data: Vec<u8>) -> Message {
        Message {
            msg_id: id,
            msg_len: data.len() as u32,
            data,
        }
    }
}
