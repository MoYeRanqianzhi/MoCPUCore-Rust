use std::io::{Read, stdin, stdout, Write};
use crate::pc::ProgramCounter;

pub struct Registers {
    pc: ProgramCounter,
    i: Box<dyn Read + 'static>,
    o: Box<dyn Write + 'static>,
    registers: [u8; 254],
}

impl Registers {
    #[inline]
    pub fn new(pc: ProgramCounter, i: Box<dyn Read + 'static>, o: Box<dyn Write + 'static>) -> Self {
        Self {
            pc,
            i,
            o,
            registers: [0; 254],
        }
    }

    #[inline]
    pub fn std() -> Self {
        Self {
            pc: ProgramCounter::new(),
            i: Box::new(stdin()),
            o: Box::new(stdout()),
            registers: [0; 254],
        }
    }
}

impl Registers {
    #[inline]
    pub fn get(&mut self, index: u8) -> u8 {
        match index {
            254 => self.pc.pos,
            255 => {
                let mut buffer: [u8; 1] = [0];
                match self.i.read(&mut buffer) {
                    Ok(_) => buffer[0],
                    Err(_) => panic!("输入流读取错误, 无法恢复!!!")
                }
            }
            _ => self.registers[index as usize]
        }
    }

    #[inline]
    pub fn set(&mut self, index: u8, value: u8) {
        match index {
            254 => self.pc.jump(value),
            255 => {
                self.o.write(&[value]).expect("输出流写入错误, 无法恢复!!!");
                self.o.flush().expect("输出流刷新失败, 无法恢复!!!");
            }
            _ => self.registers[index as usize] = value
        }
    }
}

// struct AsciiOut;
//
// impl Write for AsciiOut {
//     fn write(&mut self, buf: &[u8]) -> Result<usize> {
//         stdout().write()
//     }
//
//     fn flush(&mut self) -> Result<()> {
//         stdout().flush()
//     }
// }

