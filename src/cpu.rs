use std::io::{Read, Result};

use crate::{
    audio::Audio,
    display::Display,
    instruction::Instruction::{self, *},
};

const MEM_LIMIT: usize = 2_usize.pow(16);

pub enum Mode {
    Cosmac,
    Super,
    Xo,
}

pub struct Cpu {
    mem: [u8; MEM_LIMIT],
    pc: u16,
    index: u16,
    stack: Vec<u16>,
    delay: u8,
    sound: u8,
    audio: Audio,
    mode: Mode,
    regs: [u8; 16],
}

impl Cpu {
    pub fn with_mode(mode: Mode) -> Self {
        let mut mem = [0; MEM_LIMIT];
        // load font into memory
        mem[0x50..=0x9F].clone_from_slice(&[
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ]);
        Cpu {
            mem,
            pc: 0x200, // program code starts at 0x200
            index: 0,
            stack: Vec::new(),
            delay: 0,
            sound: 0,
            audio: Audio::new(),
            mode,
            regs: [0; 16],
        }
    }
    pub fn dec_timers(&mut self) {
        if self.delay > 0 {
            self.delay -= 1;
        }
        if self.sound > 0 {
            self.sound -= 1;
        }
        if self.sound > 0 {
            self.audio.play();
        } else {
            self.audio.pause();
        }
    }
    pub fn execute(&mut self, ins: &Instruction, disp: &mut Display) {
        match ins {
            ScrollUp(num) => match self.mode {
                Mode::Cosmac => {
                    panic!("Instruction not available in Cosmac mode. Please run in XO mode.")
                }
                Mode::Super => {
                    panic!("Instruction not available in Super mode. Please run in XO mode.")
                }
                Mode::Xo => {
                    for _ in 0..=*num {
                        disp.scroll_up();
                    }
                }
            },
            ScrollDown(num) => {
                if let Mode::Cosmac = self.mode {
                    panic!(
                        "Instruction not available in Cosmac mode. Please run in Super or XO mode."
                    )
                }
                for _ in 0..=*num {
                    disp.scroll_down();
                }
            }
            Clear => disp.clear(),
            Return => self.pc = self.stack.pop().unwrap(),
            ScrollRight => disp.scroll_right(),
            ScrollLeft => disp.scroll_left(),
            Lores => {
                match self.mode {
                    Mode::Cosmac => panic!(
                        "Instruction not available in Cosmac mode. Please run in Super or XO mode."
                    ),
                    Mode::Super => (),
                    Mode::Xo => disp.clear(),
                }
                disp.set_lores();
            }
            Hires => {
                match self.mode {
                    Mode::Cosmac => panic!(
                        "Instruction not available in Cosmac mode. Please run in Super or XO mode."
                    ),
                    Mode::Super => (),
                    Mode::Xo => disp.clear(),
                }
                disp.set_hires();
            }
            Jump(addr) => self.pc = *addr,
            Call(addr) => {
                self.stack.push(self.pc);
                self.pc = *addr;
            }
            SkipEqualConst(reg, val) => {
                if self.regs[*reg] == *val {
                    self.pc += 2;
                }
            }
            SkipUnequalConst(reg, val) => {
                if self.regs[*reg] != *val {
                    self.pc += 2;
                }
            }
            SkipEqualReg(reg1, reg2) => {
                if self.regs[*reg1] == self.regs[*reg2] {
                    self.pc += 2;
                }
            }
            SaveRange(reg1, reg2) => match self.mode {
                Mode::Cosmac => {
                    panic!("Instruction not available in Cosmac mode. Please run in XO mode.")
                }
                Mode::Super => {
                    panic!("Instruction not available in Super mode. Please run in XO mode.")
                }
                Mode::Xo => {
                    let mut i = self.index as usize;
                    for reg in self.regs.iter().take(*reg2 + 1).skip(*reg1) {
                        self.mem[i] = *reg;
                        i += 1;
                    }
                }
            },
            LoadRange(reg1, reg2) => match self.mode {
                Mode::Cosmac => {
                    panic!("Instruction not available in Cosmac mode. Please run in XO mode.")
                }
                Mode::Super => {
                    panic!("Instruction not available in Super mode. Please run in XO mode.")
                }
                Mode::Xo => {
                    let mut i = self.index as usize;
                    for reg in self.regs.iter_mut().take(*reg2 + 1).skip(*reg1) {
                        *reg = self.mem[i];
                        i += 1;
                    }
                }
            },
            SetConst(reg, val) => self.regs[*reg] = *val,
            AddConst(reg, val) => self.regs[*reg] = self.regs[*reg].wrapping_add(*val),
            SetReg(reg1, reg2) => self.regs[*reg1] = self.regs[*reg2],
            // NOTE: or, and, xor are ambiguous
            // they may or may not reset flag reg
            // undefined behavior to rely on this
            Or(reg1, reg2) => {
                self.regs[*reg1] |= self.regs[*reg2];
                self.regs[0xF] = 0;
            }
            And(reg1, reg2) => {
                self.regs[*reg1] &= self.regs[*reg2];
                self.regs[0xF] = 0;
            }
            Xor(reg1, reg2) => {
                self.regs[*reg1] ^= self.regs[*reg2];
                self.regs[0xF] = 0;
            }
            AddReg(reg1, reg2) => {
                let (sum, under) = self.regs[*reg1].overflowing_add(self.regs[*reg2]);
                self.regs[*reg1] = sum;
                self.regs[0xF] = under as u8;
            }
            SubRFromL(reg1, reg2) => {
                let (diff, under) = self.regs[*reg1].overflowing_sub(self.regs[*reg2]);
                self.regs[*reg1] = diff;
                self.regs[0xF] = !under as u8;
            }
            // NOTE:ambiguous instruction
            RShift(reg1, reg2) => {
                // this is the line that's ambiguous
                if let Mode::Cosmac = self.mode {
                    self.regs[*reg1] = self.regs[*reg2];
                }
                let flag = self.regs[*reg1] & 1;
                self.regs[*reg1] >>= 1;
                self.regs[0xF] = flag;
            }
            SubLFromR(reg1, reg2) => {
                let (diff, over) = self.regs[*reg2].overflowing_sub(self.regs[*reg1]);
                self.regs[*reg1] = diff;
                self.regs[0xF] = !over as u8;
            }
            // NOTE:ambiguous instruction
            LShift(reg1, reg2) => {
                // this is the line that's ambiguous
                if let Mode::Cosmac = self.mode {
                    self.regs[*reg1] = self.regs[*reg2];
                }
                let flag = (self.regs[*reg1] & 0x80 > 0) as u8;
                self.regs[*reg1] <<= 1;
                self.regs[0xF] = flag;
            }
            SkipUnequalReg(reg1, reg2) => {
                if self.regs[*reg1] != self.regs[*reg2] {
                    self.pc += 2;
                }
            }
            SetIndex(val) => self.index = *val,
            // TODO:ambiguous instruction; add toggle
            JumpOffset(addr) => self.pc = addr + self.regs[0] as u16,
            Rand(reg, lim) => {
                self.regs[*reg] = lim & rand::random_range(0..=0xFF);
            }
            Draw(reg_x, reg_y, height) => {
                if !disp.just_updated {
                    self.pc -= 2;
                    return;
                }
                self.regs[0xF] = 0;
                let x = self.regs[*reg_x] as usize % disp.width;
                let y = self.regs[*reg_y] as usize % disp.height;
                if *height == 0 {
                    match self.mode {
                        Mode::Cosmac => return,
                        // NOTE: Not sure if Xo mode is supposed to work
                        // like this, but I know Octo is supposed to
                        Mode::Super | Mode::Xo => {
                            // draw 16x16 sprite
                            for row in (0..32).step_by(2) {
                                let sprite = ((self.mem[self.index as usize + row] as u16) << 8)
                                    | self.mem[self.index as usize + row + 1] as u16;
                                let mut mask = 0x8000;
                                for col in 0..16 {
                                    if sprite & mask > 0 && disp.draw_at(x + col, y + (row / 2)) {
                                        self.regs[0xF] = 1;
                                    }
                                    mask >>= 1;
                                }
                            }
                        }
                    }
                }
                // draw normal sized 8x8 sprite
                for row in 0..*height {
                    let sprite = self.mem[self.index as usize + row];
                    let mut mask = 0x80;
                    for col in 0..8 {
                        if sprite & mask > 0 && disp.draw_at(x + col, y + row) {
                            self.regs[0xF] = 1;
                        }
                        mask >>= 1;
                    }
                }
            }
            SkipKey(reg) => {
                if disp.key_pressed(self.regs[*reg]) {
                    self.pc += 2;
                }
            }
            SkipNotKey(reg) => {
                if !disp.key_pressed(self.regs[*reg]) {
                    self.pc += 2;
                }
            }
            SetIndexWide => match self.mode {
                Mode::Cosmac => {
                    panic!("Instruction not available in Cosmac mode. Please run in XO mode.")
                }
                Mode::Super => {
                    panic!("Instruction not available in Super mode. Please run in XO mode.")
                }
                Mode::Xo => {
                    self.pc += 2;
                    self.index = ((self.mem[(self.pc - 2) as usize] as u16) << 8)
                        | self.mem[(self.pc - 1) as usize] as u16;
                }
            },
            Audio => match self.mode {
                Mode::Cosmac => {
                    panic!("Instruction not available in Cosmac mode. Please run in XO mode.")
                }
                Mode::Super => {
                    panic!("Instruction not available in Super mode. Please run in XO mode.")
                }
                Mode::Xo => self.audio.set_pattern(
                    self.mem[(self.index as usize)..(self.index as usize + 16)]
                        .try_into()
                        .unwrap(),
                ),
            },
            GetDelay(reg) => self.regs[*reg] = self.delay,
            GetKey(reg) => {
                if disp.just_pressed_key && !disp.key_pressed(self.regs[*reg]) {
                    return;
                } else if let Some(i) = (0..16).find(|&i| disp.key_pressed(i)) {
                    self.regs[*reg] = i;
                }
                self.pc -= 2;
            }
            SetDelay(reg) => self.delay = self.regs[*reg],
            SetSound(reg) => self.sound = self.regs[*reg],
            AddIndex(reg) => self.index = self.index.wrapping_add(self.regs[*reg] as u16),
            Font(reg) => self.index = ((self.regs[*reg] & 0xF) * 5) as u16 + 0x50,
            ConvertToDecimal(reg) => {
                self.mem[self.index as usize] = self.regs[*reg] / 100;
                self.mem[self.index as usize + 1] = self.regs[*reg] / 10 % 10;
                self.mem[self.index as usize + 2] = self.regs[*reg] % 10;
            }
            SetPitch(pitch) => self.audio.set_pitch(*pitch),
            // NOTE:ambiguous instruction
            Store(r) => {
                let mut i = self.index as usize;
                for reg in self.regs.iter().take(*r + 1) {
                    self.mem[i] = *reg;
                    i += 1;
                }
                if let Mode::Cosmac = self.mode {
                    self.index += *r as u16 + 1;
                }
            }
            // NOTE: ambiguous instruction
            Load(r) => {
                let mut i = self.index as usize;
                for reg in self.regs.iter_mut().take(*r + 1) {
                    *reg = self.mem[i];
                    i += 1;
                }
                if let Mode::Cosmac = self.mode {
                    self.index += *r as u16 + 1;
                }
            }
        }
    }
    pub fn fetch(&mut self) -> Instruction {
        self.pc += 2;
        Instruction::from(
            ((self.mem[(self.pc - 2) as usize] as u16) << 8)
                | self.mem[(self.pc - 1) as usize] as u16,
        )
    }
    pub fn load<R: Read>(&mut self, input: &mut R) -> Result<usize> {
        input.read(&mut self.mem[(self.pc as usize)..])
    }
}
