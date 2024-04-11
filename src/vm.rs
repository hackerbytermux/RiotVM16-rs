pub mod librt;

use librt::{VMM, CPU, Tools};
use std::io;

pub fn get_vm(bytecode: &[u8]) -> CPU {

        let mut vmm = VMM::new(8192);
    
        vmm.alloc(&bytecode);
    
        let mut cpu = CPU::new(vmm, 8);
        //opcodes

        //vm_print (stack: int)
        cpu.opcode(0x01, |cpu| {
            let value = cpu.stack.pop_front().unwrap();
            println!("{}", value);
        });

        //vm_prints (stack: addr)
        cpu.opcode(0x02, |cpu| {
            let value = cpu.stack.pop_front().unwrap();

            let mut buf: String = "".to_string();
            let mut ptr = value;

            loop {
                let byte = cpu.memory.read(ptr.into());
                if byte == 0 {
                    break;
                }
                buf += char::from(byte).to_string().as_str();
                ptr += 1;
            }
            
            println!("{}", buf);
        });

        //vm_input () -> register[3]
        cpu.opcode(0x03, | cpu| {
            let mut input_line = String::new();
            io::stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");
            let x: u16 = input_line.trim().parse().expect("Input not an integer");
            cpu.registers[3] = x;
        });
        
        //vm_inputs (stack: addr)
        cpu.opcode(0x04, |cpu| {
            let addr: usize = cpu.stack.pop_front().unwrap().into();

            let mut input_line = String::new();
            io::stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");
            for i in 0..input_line.len() {
                cpu.memory.write(addr + i, input_line.as_bytes()[i]);
            }
            cpu.memory.write(addr+input_line.len(), 0);
        });

        //mov_0
        //mov (memory: int)
        cpu.opcode(0xC0, |cpu| {
            cpu.ptr += 1;
            let register = cpu.memory.read(cpu.ptr);
            cpu.ptr += 1;
            let value = Tools::read_bytes(cpu, cpu.ptr, 2);
            cpu.registers[register as usize] = Tools::bytes_to_u16(value[0], value[1]);
            cpu.ptr += 1;
        });


        cpu.opcode(0xC1, |cpu| {
            cpu.ptr += 1;
            let register = cpu.memory.read(cpu.ptr);
            cpu.ptr += 1;
            let value = cpu.memory.read(cpu.ptr);
            cpu.registers[register as usize] = cpu.registers[value as usize];
        });
        
        cpu.opcode(0xC2, |cpu| {
            cpu.ptr += 1;
            let register = cpu.memory.read(cpu.ptr);
            cpu.stack.push_back(cpu.registers[register as usize]);
        });

        //push num
        cpu.opcode(0xC3, | cpu | {
            cpu.ptr += 1;
            let value = Tools::read_bytes(cpu, cpu.ptr, 2);
            cpu.stack.push_back(Tools::bytes_to_u16(value[0], value[1]));
            cpu.ptr += 1
        });


        //pop
        cpu.opcode(0x29, | cpu | {
            cpu.ptr += 1;
            let register = cpu.memory.read(cpu.ptr);
            cpu.registers[register as usize] = cpu.stack.pop_back().unwrap();
        });

        //inc
        cpu.opcode(0x30, | cpu | {
            cpu.ptr += 1;
            let register = cpu.memory.read(cpu.ptr);
            cpu.registers[register as usize] += 1;
        });

        //dec
        cpu.opcode(0x31, | cpu | {
            cpu.ptr += 1;
            let register = cpu.memory.read(cpu.ptr);
            cpu.registers[register as usize] -= 1;
        });


        //add
        cpu.opcode(0x32, | cpu | {
            cpu.ptr += 1;
            let register1 = cpu.memory.read(cpu.ptr);
            cpu.ptr += 1;
            let register2 = cpu.memory.read(cpu.ptr);
            cpu.registers[register1 as usize] += cpu.registers[register2 as usize];
        });

        //sub
        cpu.opcode(0x33, | cpu | {
            cpu.ptr += 1;
            let register1 = cpu.memory.read(cpu.ptr);
            cpu.ptr += 1;
            let register2 = cpu.memory.read(cpu.ptr);
            cpu.registers[register1 as usize] -= cpu.registers[register2 as usize];
        });

        //mult
        cpu.opcode(0x34, | cpu | {
            cpu.ptr += 1;
            let register1 = cpu.memory.read(cpu.ptr);
            cpu.ptr += 1;
            let register2 = cpu.memory.read(cpu.ptr);
            cpu.registers[register1 as usize] *= cpu.registers[register2 as usize];
        });

        //div
        cpu.opcode(0x35, | cpu | {
            cpu.ptr += 1;
            let register1 = cpu.memory.read(cpu.ptr);
            cpu.ptr += 1;
            let register2 = cpu.memory.read(cpu.ptr);
            cpu.registers[register1 as usize] /= cpu.registers[register2 as usize];
        });

        //xor
        cpu.opcode(0x36, | cpu | {
            cpu.ptr += 1;
            let register1 = cpu.memory.read(cpu.ptr);
            cpu.ptr += 1;
            let register2 = cpu.memory.read(cpu.ptr);
            cpu.registers[register1 as usize] ^= cpu.registers[register2 as usize];
        });

        //cmp
        cpu.opcode(0xD0, | cpu | {
            cpu.ptr += 1;
            let register1 = cpu.memory.read(cpu.ptr);
            cpu.ptr += 1;
            let register2 = cpu.memory.read(cpu.ptr);
            cpu.registers[7] = (cpu.registers[register1 as usize] == cpu.registers[register2 as usize]) as u16;
        });


        //strcmp
        cpu.opcode(0xD1, | cpu | {
            let addr1: usize = cpu.stack.pop_front().unwrap() as usize;
            let addr2: usize = cpu.stack.pop_front().unwrap() as usize;
            let mut n: usize = 0;
            while cpu.memory.memory[addr1+n] != 0{  
                let a = cpu.memory.memory[addr1+n];
                let b = cpu.memory.memory[addr2+n];
                if a != b{
                    cpu.registers[7] = 0;
                    return
                }
                n += 1;
            }
            cpu.registers[7] = 1;
        });

        //jmp
        cpu.opcode(0xF0, | cpu | {
            let addr: usize = cpu.stack.pop_front().unwrap().into();
            cpu.registers[6] = cpu.ptr as u16;
            cpu.ptr = addr - 1;
        });

        //je
        cpu.opcode(0xF1, | cpu | {
            let addr: usize = cpu.stack.pop_front().unwrap().into();
            cpu.registers[6] = cpu.ptr as u16;
            if cpu.registers[7] == 1 {
                cpu.ptr = addr - 1;
            }
        });

        //jne
        cpu.opcode(0xF2, | cpu | {
            let addr: usize = cpu.stack.pop_front().unwrap().into();
            cpu.registers[6] = cpu.ptr as u16;
            if cpu.registers[7] == 0 {
                cpu.ptr = addr - 1;
            }
        });

        //read memory (addr: stack)
        cpu.opcode(0x1E, | cpu | {
            let addr: usize = cpu.stack.pop_front().unwrap().into();
            cpu.registers[3] = cpu.memory.read(addr) as u16;
        });
        
        //write memory(addr: stack)
        cpu.opcode(0x1F, | cpu | {
            let addr: usize = cpu.stack.pop_front().unwrap().into();
            cpu.memory.write(addr, cpu.registers[7] as u8);
        });

        //ret
        cpu.opcode(0xFE, |cpu| {
            cpu.ptr = cpu.registers[6] as usize;
        });
        
        cpu.opcode(0xFF, |cpu| {
            cpu.ptr = cpu.memory.size;
        });

        cpu
}