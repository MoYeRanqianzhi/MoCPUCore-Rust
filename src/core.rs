use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use crate::pc::ProgramCounter;
use crate::registers::Registers;

pub struct MoCPU {
    pc: ProgramCounter,
    tick: f64,
    registers: Registers,
    operations: HashMap<u8, Box<dyn Fn(u8, u8) -> u8>>,
    conditions: HashMap<u8, Box<dyn Fn(u8, u8) -> bool>>,
}

impl MoCPU {
    #[inline]
    pub fn new(
        pc: ProgramCounter,
        tick: f64,
        registers: Registers,
    ) -> Self {
        let ops: Vec<(u8, Box<dyn Fn(u8, u8) -> u8>)> = vec![
            (0, Box::new(|a, b| a + b)),
            (1, Box::new(|a, b| a - b)),
            (2, Box::new(|a, b| a * b)),
            (3, Box::new(|a, b| a / b)),
            (4, Box::new(|a, b| a / b)),
            (5, Box::new(|a, b| a % b)),
            (6, Box::new(|a, b| a.pow(b as u32))),
            (7, Box::new(|a, b| a & b)),
            (8, Box::new(|a, b| a | b)),
            (9, Box::new(|a, b| a ^ b)),
        ];
        let cos: Vec<(u8, Box<dyn Fn(u8, u8) -> bool>)> = vec![
            (32, Box::new(|a, b| true)),
            (33, Box::new(|a, b| false)),
            (34, Box::new(|a, b| a == b)),
            (35, Box::new(|a, b| a != b)),
            (36, Box::new(|a, b| a < b)),
            (37, Box::new(|a, b| a <= b)),
            (38, Box::new(|a, b| a > b)),
            (39, Box::new(|a, b| a >= b)),
        ];

        let operations: HashMap<u8, Box<dyn Fn(u8, u8) -> u8>> = ops.into_iter().collect();
        let conditions: HashMap<u8, Box<dyn Fn(u8, u8) -> bool>> = cos.into_iter().collect();

        Self {
            pc,
            tick,
            registers,
            operations,
            conditions,
        }
    }

    #[inline]
    pub fn std() -> Self {
        Self::new(ProgramCounter::new(), 0.0, Registers::std())
    }
}

impl MoCPU {
    #[inline]
    pub fn run(&mut self, code: [[u8; 4]; 256]) {
        while self.pc.pos < 255 {
            self.exec(
                code[self.pc.pos as usize][0],
                code[self.pc.pos as usize][1],
                code[self.pc.pos as usize][2],
                code[self.pc.pos as usize][3],
            );
            self.pc.next();

            if self.tick > 0.0 {
                sleep(Duration::from_secs_f64(self.tick));
            }
        }
    }

    #[inline]
    fn exec(&mut self, opcode: u8, value1: u8, value2: u8, target: u8) {
        let op = opcode & 0b0011_1111;
        let v1 = if opcode & 128 == 128 { value1 } else { self.registers.get(value1) };
        let v2 = if opcode & 64 == 64 { value2 } else { self.registers.get(value2) };
        if op & 32 == 0 {
            let operation = self.operations.get(&op).expect("无效的操作码");
            self.registers.set(target, operation(v1, v2));
        } else {
            let condition = self.conditions.get(&op).expect("无效的条件码");
            if condition(v1, v2) {
                self.pc.jump(target);
            }
        }
    }
}