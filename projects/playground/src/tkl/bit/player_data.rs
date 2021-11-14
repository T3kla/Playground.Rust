use super::span::*;
// use std::fmt::*;

pub const SPN_HLTH: Span = Span { sft: 24, len: 8 };
pub const SPN_BLTS: Span = Span { sft: 16, len: 8 };
pub const SPN_BRTHS: Span = Span { sft: 12, len: 4 };
// pub const SPN_EMP: Span = Span { sft: 4, len: 8 };
pub const SPN_BRSK: Span = Span { sft: 3, len: 1 };
pub const SPN_SHLD: Span = Span { sft: 2, len: 1 };
pub const SPN_INFBULTS: Span = Span { sft: 1, len: 1 };
pub const SPN_GOD: Span = Span { sft: 0, len: 1 };

pub struct PlayerData {
    pub data: i32,
}

impl PlayerData {
    pub fn get(&self, spn: &Span) -> i32 {
        (self.data & spn.mask()) >> spn.sft
    }
    pub fn set(&mut self, spn: &Span, val: i32) {
        self.clr(spn);
        self.data |= (val << spn.sft) & spn.mask();
    }
    pub fn clr(&mut self, spn: &Span) {
        self.data &= !(self.data & spn.mask());
    }
    pub fn add(&mut self, spn: &Span, val: i32) {
        let buff: i32 = self.get(spn) + val;
        self.clr(spn);
        self.set(spn, buff)
    }
}

impl Default for PlayerData {
    fn default() -> Self {
        Self { data: 0 }
    }
}

// impl Debug for PlayerData {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         f.debug_struct("struct")
//             .field("health", &self.spn_hlt)
//             .field("len", &self.len)
//             .finish()
//     }
// }
