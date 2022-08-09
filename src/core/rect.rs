use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rect {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

impl From<(i32, i32, i32, i32)> for Rect {
    fn from(rect: (i32, i32, i32, i32)) -> Self {
        Self {
            left: rect.0,
            right: rect.2,
            top: rect.1,
            bottom: rect.3,
        }
    }
}
