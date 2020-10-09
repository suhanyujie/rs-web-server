//! 数据封包、解包抽象

use crate::interface::imessage::IMessage;
use std::u8;

pub trait IDataPack {
    // 获取封包时的header长度
    fn get_header_len(&self) -> u32;
    fn pack(&self, msg: impl IMessage)-> Result<Vec<u8>, String>;
    fn unpack(&self, binary_data: Vec<u8>) -> Box<dyn IMessage>;
}
