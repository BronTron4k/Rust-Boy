use super::cpu;

#[derive(Debug, Default)]
pub struct GameBoy {
    cpu: cpu::Cpu,
}

impl GameBoy {
    pub fn new() -> Self {
        GameBoy::default()
    }
    pub fn run(&mut self) {
       self.cpu.run();
    }
}
