use core::arch::asm;

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: usize,
    pub sepc: usize,
}

impl TrapContext {
    pub fn new(entry: usize, task_sp: usize) -> Self {
        let mut sstatus;
        unsafe {
            asm! {
            "csrr {}, sstatus",
            out(reg) sstatus,
            }
        }
        clear_spp(&mut sstatus);
        set_sum(&mut sstatus);

        let mut task_context = TrapContext {
            x: [0; 32],
            sstatus,
            sepc: entry
        };
        task_context.x[2] = task_sp;
        task_context
    }
}

fn clear_spp(sstatus: &mut usize) {
    *sstatus &= usize::MAX - (1 << 8);
}

fn set_sum(sstatus: &mut usize) {
    *sstatus |= 1 << 18;
}