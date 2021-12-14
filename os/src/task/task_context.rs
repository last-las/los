use riscv::register::{sstatus::{self, SPP}};
use riscv::register::sstatus::Sstatus;

#[repr(C)]
pub struct TaskContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl TaskContext {
    pub fn new(entry: usize, task_sp: usize, is_user: bool) -> Self {
        let mut sstatus = sstatus::read();
        if is_user {
            sstatus.set_spp(SPP::User);
        } else {
            sstatus.set_spp(SPP::Supervisor);
        }
        let mut task_context = TaskContext {
            x: [0; 32],
            sstatus,
            sepc: entry
        };
        task_context.x[2] = task_sp;
        task_context
    }
}