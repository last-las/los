use core::arch::asm;

#[repr(C)]
#[derive(Debug)]
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

        // The sstatus.sum bit is only available on risc-v 1.11. K210 risc-v version is 1.9, so
        // this bit is only set on qemu platform right now.
        #[cfg(feature = "board_qemu")]
        set_sum(&mut sstatus);

        let mut task_context = TrapContext {
            x: [0; 32],
            sstatus,
            sepc: entry
        };
        task_context.x[2] = task_sp;
        task_context
    }

    pub fn clone(&self) -> Self {
        Self {
            x: self.x,
            sstatus: self.sstatus,
            sepc: self.sepc,
        }
    }
}

fn clear_spp(sstatus: &mut usize) {
    *sstatus &= usize::MAX - (1 << 8);
}

#[cfg(feature = "board_qemu")]
fn set_sum(sstatus: &mut usize) {
    *sstatus |= 1 << 18;
}