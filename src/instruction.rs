use Instruction::*;

pub enum Instruction {
    ScrollDown(u16),
    ScrollUp(u16),
    Clear,
    Return,
    ScrollRight,
    ScrollLeft,
    Lores,
    Hires,
    Jump(u16),
    Call(u16),
    SkipEqualConst(usize, u8),
    SkipUnequalConst(usize, u8),
    SkipEqualReg(usize, usize),
    SaveRange(usize, usize),
    LoadRange(usize, usize),
    SetConst(usize, u8),
    AddConst(usize, u8),
    SetReg(usize, usize),
    Or(usize, usize),
    And(usize, usize),
    Xor(usize, usize),
    AddReg(usize, usize),
    SubRFromL(usize, usize),
    RShift(usize, usize),
    SubLFromR(usize, usize),
    LShift(usize, usize),
    SkipUnequalReg(usize, usize),
    SetIndex(u16),
    JumpOffset(u16),
    Rand(usize, u8),
    Draw(usize, usize, usize),
    SkipKey(usize),
    SkipNotKey(usize),
    SetIndexWide,
    Audio,
    GetDelay(usize),
    GetKey(usize),
    SetDelay(usize),
    SetSound(usize),
    AddIndex(usize),
    Font(usize),
    ConvertToDecimal(usize),
    SetPitch(u16),
    Store(usize),
    Load(usize),
}

impl std::convert::From<u16> for Instruction {
    fn from(word: u16) -> Self {
        // println!("{:#06x}", word);
        match word {
            0x00C0..=0x00CF => ScrollDown(word & 0xF),
            0x00D0..=0x00DF => ScrollUp(word & 0xF),
            0x00E0 => Clear,
            0x00EE => Return,
            0x00FB => ScrollRight,
            0x00FC => ScrollLeft,
            0x00FE => Lores,
            0x00FF => Hires,
            0x1000..=0x1FFF => Jump(word & 0xFFF),
            0x2000..=0x2FFF => Call(word & 0xFFF),
            0x3000..=0x3FFF => SkipEqualConst(((word & 0xF00) >> 8) as usize, (word & 0xFF) as u8),
            0x4000..=0x4FFF => {
                SkipUnequalConst(((word & 0xF00) >> 8) as usize, (word & 0xFF) as u8)
            }
            0x5000..=0x5FFF => match word & 0xF {
                0 => SkipEqualReg(
                    ((word & 0xF00) >> 8) as usize,
                    ((word & 0xF0) >> 4) as usize,
                ),
                2 => SaveRange(
                    ((word & 0xF00) >> 8) as usize,
                    ((word & 0xF0) >> 4) as usize,
                ),
                3 => LoadRange(
                    ((word & 0xF00) >> 8) as usize,
                    ((word & 0xF0) >> 4) as usize,
                ),
                _ => panic!("{:#06x} is not an instruction", word),
            },
            0x6000..=0x6FFF => SetConst(((word & 0xF00) >> 8) as usize, word as u8),
            0x7000..=0x7FFF => AddConst(((word & 0xF00) >> 8) as usize, word as u8),
            0x8000..=0x8FFF => match word & 0xF {
                0 => SetReg(
                    ((word & 0xF00) >> 8) as usize,
                    ((word & 0xF0) >> 4) as usize,
                ),
                1 => Or(
                    ((word & 0xF00) >> 8) as usize,
                    ((word & 0xF0) >> 4) as usize,
                ),
                2 => And(
                    ((word & 0xF00) >> 8) as usize,
                    ((word & 0xF0) >> 4) as usize,
                ),
                3 => Xor(
                    ((word & 0xF00) >> 8) as usize,
                    ((word & 0xF0) >> 4) as usize,
                ),
                4 => AddReg(
                    ((word & 0xF00) >> 8) as usize,
                    ((word & 0xF0) >> 4) as usize,
                ),
                5 => SubRFromL(
                    ((word & 0xF00) >> 8) as usize,
                    ((word & 0xF0) >> 4) as usize,
                ),
                6 => RShift(
                    ((word & 0xF00) >> 8) as usize,
                    ((word & 0xF0) >> 4) as usize,
                ),
                7 => SubLFromR(
                    ((word & 0xF00) >> 8) as usize,
                    ((word & 0xF0) >> 4) as usize,
                ),
                0xE => LShift(
                    ((word & 0xF00) >> 8) as usize,
                    ((word & 0xF0) >> 4) as usize,
                ),
                _ => panic!("{:#06x} is not an instruction", word),
            },
            0x9000..=0x9FFF => match word & 0xF {
                0 => SkipUnequalReg(
                    ((word & 0xF00) >> 8) as usize,
                    ((word & 0xF0) >> 4) as usize,
                ),
                _ => panic!("{:#06x} is not an instruction", word),
            },
            0xA000..=0xAFFF => SetIndex(word & 0xFFF),
            0xB000..=0xBFFF => JumpOffset(word & 0xFFF),
            0xC000..=0xCFFF => Rand(((word & 0xF00) >> 8) as usize, word as u8),
            0xD000..=0xDFFF => Draw(
                ((word & 0xF00) >> 8) as usize,
                ((word & 0xF0) >> 4) as usize,
                (word & 0xF) as usize,
            ),
            0xE000..=0xEFFF => match word & 0xFF {
                0x9E => SkipKey(((word & 0xF00) >> 8) as usize),
                0xA1 => SkipNotKey(((word & 0xF00) >> 8) as usize),
                _ => panic!("{:#06x} is not an instruction", word),
            },
            0xF000 => SetIndexWide,
            0xF002 => Audio,
            0xF001..=0xFFFF => match word & 0xFF {
                0x07 => GetDelay(((word & 0xF00) >> 8) as usize),
                0x0A => GetKey(((word & 0xF00) >> 8) as usize),
                0x15 => SetDelay(((word & 0xF00) >> 8) as usize),
                0x18 => SetSound(((word & 0xF00) >> 8) as usize),
                0x1E => AddIndex(((word & 0xF00) >> 8) as usize),
                0x29 => Font(((word & 0xF00) >> 8) as usize),
                // 0x30 =>
                0x33 => ConvertToDecimal(((word & 0xF00) >> 8) as usize),
                0x3A => SetPitch((word & 0xF00) >> 8),
                0x55 => Store(((word & 0xF00) >> 8) as usize),
                0x65 => Load(((word & 0xF00) >> 8) as usize),
                _ => panic!("{:#06x} is not an instruction", word),
            },
            _ => panic!("{:#06x} is not an instruction", word),
        }
    }
}
