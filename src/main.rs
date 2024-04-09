pub mod vm;

use vm::{get_vm};
// +2
fn main() {
    let program = [
        0xC3, 0x02, 0x00,
        0xF0,
    ];

    let mut cpu = get_vm(&program);
    cpu.run();
    println!("{:?}", cpu.registers);

}   