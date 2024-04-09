use std::collections::VecDeque;

pub struct VMM {
    pub memory: Vec<u8>,
    pub size: usize,
}

impl VMM {
    pub fn new(size: usize) -> Self {
        VMM {
            memory: vec![0; size],
            size,
        }
    }

    pub fn alloc(&mut self, bytecode: &[u8]) {
        for (i, &byte) in bytecode.iter().enumerate() {
            self.memory[i] = byte;
        }
    }

    pub fn write(&mut self, ptr: usize, value: u8) {
        self.memory[ptr] = value;
    }

    pub fn read(&self, ptr: usize) -> u8 {
        self.memory[ptr]
    }

    pub fn read_bytes(&self, start: usize, end: usize) -> Vec<u8> {
        self.memory[start..end].to_vec()
    }

    pub fn reset(&mut self) {
        self.memory = vec![0; self.size];
    }
}

pub struct CPU {
    pub memory: VMM,
    pub opcodes: std::collections::HashMap<u8, fn(&mut CPU)>,
    pub ptr: usize,
    pub registers: Vec<u16>,
    pub reg_sz: usize,
    pub stack: VecDeque<u16>,
}

impl CPU {
    pub fn new(memory: VMM, reg_sz: usize) -> Self {
        CPU {
            memory,
            opcodes: std::collections::HashMap::new(),
            ptr: 0,
            registers: vec![0; reg_sz],
            reg_sz,
            stack: VecDeque::new(),
        }
    }

    pub fn opcode(&mut self, value: u8, func: fn(&mut CPU)) {
        self.opcodes.insert(value, func);
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.memory.alloc(program);
    }

    pub fn run(&mut self) {
        while self.ptr < self.memory.memory.len() {
            let opcode = self.memory.read(self.ptr);
            if let Some(func) = self.opcodes.get(&opcode) {
                func(self);
            }
            self.ptr += 1;
        }
    }

    pub fn reset(&mut self) {
        self.registers = vec![0; self.reg_sz];
        self.stack = VecDeque::new();
        self.ptr = 0;
        self.memory.reset();
    }
}

pub struct Tools;

impl Tools {
    pub fn bytes_to_int(byte: &[u8]) -> u64 {
        let mut result: u64 = 0;
        for &b in byte.iter().rev() {
            result = (result << 8) + u64::from(b);
        }
        result
    }

    pub fn int_to_bytes(mut number: u64, size: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(size);
        for _ in 0..size {
            result.push((number & 0xff) as u8);
            number >>= 8;
        }
        result
    }

    pub fn read_bytes(cpu: &CPU, index: usize, size: usize) -> Vec<u8> {
        cpu.memory.read_bytes(index, index + size)
    }

    pub fn bytes_to_u16(high_byte: u8, low_byte: u8) -> u16 {
        let result = (high_byte as u16) << 8 | (low_byte as u16);
        result
    }

    pub fn u16_to_bytes(value: u16) -> (u8, u8) {
        let high_byte = (value >> 8) as u8;
        let low_byte = (value & 0xFF) as u8;
        (high_byte, low_byte)
    }
}