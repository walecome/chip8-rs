mod chip8;
use chip8::cpu::Cpu;


fn main() {
    let mut cpu = Cpu::new();
    loop {
        let raw_instruction = cpu.fetch();
        let instruction = cpu.decode(raw_instruction);
        cpu.execute(instruction);
    }
}
