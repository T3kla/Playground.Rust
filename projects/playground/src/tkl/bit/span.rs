use std::fmt::*;

#[derive(Copy, Clone)]
pub struct Span {
    pub sft: u8,
    pub len: u8,
}

impl Span {
    // pub fn new() -> Self {
    //     Self { sft: 0, len: 0 }
    // }

    // pub fn from(_sft: u8, _len: u8) -> Self {
    //     Self {
    //         sft: _sft,
    //         len: _len,
    //     }
    // }

    pub fn mask(&self) -> i32 {
        let mut mask: i32 = 0;
        for i in 0..self.len {
            mask += 2i32.pow(i as u32);
        }
        mask << self.sft
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("struct")
            .field("shift", &self.sft)
            .field("len", &self.len)
            .finish()
    }
}
