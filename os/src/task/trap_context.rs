use riscv::register::{sstatus::{self, SPP}};
use riscv::register::sstatus::Sstatus;

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl TrapContext {
    pub fn new(entry: usize, task_sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut task_context = TrapContext {
            x: [0; 32],
            sstatus,
            sepc: entry
        };
        task_context.x[2] = task_sp;
        task_context
    }
}