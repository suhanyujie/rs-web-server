pub trait IMessage {
    fn set_msg_id(&mut self, id: u32);
    fn get_msg_id(&self) -> u32;
    fn set_msg_len(&mut self, len: u32);
    fn get_msg_len(&self) -> u32;
    fn set_data(&mut self, data: Vec<u8>);
    fn get_data(&self) -> &[u8];
}
