mod legacy;

pub use legacy::{sbi_console_putchar, sbi_shutdown};
pub use ipi::sbi_send_ipi;

#[inline(always)]
fn sbi_call(eid: usize, fid: usize, args: [usize; 3]) -> SbiRet{
    let mut sbi_ret: SbiRet = SbiRet::empty();
    unsafe {
        asm!(
            "ecall",
            in("a7") eid,
            in("a6") fid,
            inout("a0") args[0] => sbi_ret.error,
            inout("a1") args[1] => sbi_ret.value,
            in("a2") args[2],
        );
    }
    sbi_ret
}

#[repr(C)]
pub struct SbiRet {
    pub error: usize,
    pub value: usize,
}

impl SbiRet {
    pub fn empty() -> Self{
        Self {
            error: 0,
            value: 0,
        }
    }
}

pub mod ipi {
    use crate::sbi::{SbiRet, sbi_call};

    const EID_PIP_EXTENSION: usize = 0x735049;
    const FID_SBI_SEND_IPI: usize = 0;

    pub fn sbi_send_ipi(hart_mask: usize, hart_mask_base: usize) -> SbiRet {
        sbi_call(EID_PIP_EXTENSION, FID_SBI_SEND_IPI, [hart_mask, hart_mask_base, 0])
    }
}