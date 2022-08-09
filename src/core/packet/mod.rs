use serde::{Deserialize, Serialize};

use super::rect::Rect;

#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    magic: u32,
    data_len: u32,
    data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    /// 处理事件
    Draw(Rect),
    /// 打开新进程
    Spawn,
    /// 捕获图像
    Capture(Rect),
    /// 其他消息
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Replay {
    message: Message,
    payload: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use crate::core::{
        packet::{Message, Replay},
        rect::Rect,
    };

    #[test]
    fn test_size() {
        let data = bincode::serialize(&Message::Draw(Rect::from((1, 2, 3, 4)))).unwrap();
        println!("{:?}", data);
    }
}
