use crate::interface::idatapack::IDataPack;
use crate::interface::imessage::IMessage;
use crate::znet::message::Message;

use std::io::{Write, Read};
use std::io::BufReader;

pub struct DataPack {}

impl IDataPack for DataPack {
    fn get_header_len(&self) -> u32 {
        // 长度（4字节） + ID（4字节）
        return 8;
    }

    /// 数据封包。
    /// 或取包头的长度，通过该长度对应的内存空间存储真实数据的大小和id信息
    fn pack(&self, msg: impl IMessage) -> Result<Vec<u8>, String> {
        let buff: Vec<u8> = vec![];
        let mut buff_writer = std::io::BufWriter::new(buff);
        // 写入长度
        let bytes1 = (msg.get_msg_len() as u8).to_be_bytes();
        let _ = buff_writer.write(&bytes1);
        // 写入id
        let _ = buff_writer.write(&[msg.get_msg_id() as u8]);
        // 写入数据
        let _ = buff_writer.write(msg.get_data());

        return Ok(buff_writer.into_inner().unwrap());
    }

    // 从字节数组中解包，得到消息
    fn unpack(&self, binary_data: Vec<u8>) -> Box<dyn IMessage> {
        let mut msg_reader = BufReader::new(&binary_data[..]);
        let mut msg_len = [0; 4];
        let _ = msg_reader.read_exact(&mut msg_len);
        let msg_len = u32::from_be_bytes(msg_len);
        let mut msg_id = [0; 4];// test
        let _ = msg_reader.read_exact(&mut msg_id);
        let msg_id = u32::from_be_bytes(msg_id);
        
        Box::new(Message {
            msg_id,
            msg_len,
            data: binary_data,
        })
    }
}
 
impl DataPack {
    pub fn new() -> DataPack {
        DataPack{

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack() {
        let dp = DataPack::new();
        let b_data = vec![97, 98, 99];
        let msg = Message::new(1, b_data);
        match dp.pack(msg) {
            Ok(bytes1) => {
                assert_eq!(vec![3, 1, 97, 98, 99], bytes1);
            },
            Err(err) => {
                eprintln!("{}", err);
                assert!(false);
            },
        }
    }
}
