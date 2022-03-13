mod legacy;

pub use legacy::{sbi_console_putchar, sbi_shutdown, sbi_legacy_send_ipi, sbi_legacy_set_timer};
pub use ipi::sbi_send_ipi;
use core::fmt::{Debug, Formatter};

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

const SBI_SUCCESS: usize = 0;
const SBI_ERR_FAILED: usize = isize::from_le(-1) as usize;
const SBI_ERR_NOT_SUPPORTED: usize = isize::from_le(-2) as usize;
const SBI_ERR_INVALID_PARAM: usize = isize::from_le(-3) as usize;
const SBI_ERR_DENIED: usize = isize::from_le(-4) as usize;
const SBI_ERR_INVALID_ADDRESS: usize = isize::from_le(-5) as usize;
const SBI_ERR_ALREADY_AVAILABLE: usize = isize::from_le(-6) as usize;
const SBI_ERR_ALREADY_STARTED: usize = isize::from_le(-7) as usize;
const SBI_ERR_ALREADY_STOPPED: usize = isize::from_le(-8) as usize;

impl SbiRet {
    pub fn empty() -> Self{
        Self {
            error: 0,
            value: 0,
        }
    }
}

impl Debug for SbiRet {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.error {
            SBI_SUCCESS => write!(f, "{:?}", self.value),
            SBI_ERR_FAILED => write!(f, "<SBI call failed>"),
            SBI_ERR_NOT_SUPPORTED => write!(f, "<SBI feature not supported>"),
            SBI_ERR_INVALID_PARAM => write!(f, "<SBI invalid parameter>"),
            SBI_ERR_DENIED => write!(f, "<SBI denied>"),
            SBI_ERR_INVALID_ADDRESS => write!(f, "<SBI invalid address>"),
            SBI_ERR_ALREADY_AVAILABLE => write!(f, "<SBI already available>"),
            SBI_ERR_ALREADY_STARTED => write!(f, "<SBI already started>"),
            SBI_ERR_ALREADY_STOPPED => write!(f, "<SBI already stopped>"),
            unknown => write!(f, "[SBI Unknown error: {}]", unknown),
        }
    }
}

// This extension is not implemented in rust-sbi yet.
#[allow(unused)]
pub mod timer {
    use crate::sbi::{sbi_call, SbiRet};

    const EID_TIMER_EXTENSION: usize = 0x54494D45;
    const FID_SET_TIMER: usize = 0;

    pub fn sbi_set_timer(stime_value: usize) -> SbiRet{
        sbi_call(EID_TIMER_EXTENSION, FID_SET_TIMER, [stime_value, 0, 0])
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

pub mod hart {
    use crate::sbi::{SbiRet, sbi_call};

    const EID_HART_STATE_MANAGEMENT_EXTENSION: usize = 0x48534D;
    #[allow(unused)]
    const FID_HART_START: usize = 0;
    #[allow(unused)]
    const FID_HART_STOP: usize = 1;
    #[allow(unused)]
    const FID_HART_GET_STATUS: usize = 2;
    const FID_HART_SUSPEND: usize = 3;

    #[allow(unused)]
    pub fn sbi_hart_get_status(hartid: usize) -> SbiRet {
        sbi_call( EID_HART_STATE_MANAGEMENT_EXTENSION, FID_HART_GET_STATUS,[hartid, 0, 0])
    }

    pub fn sbi_hart_suspend(suspend_type: usize, resume_addr: usize, opaque: usize) -> SbiRet {
        sbi_call(EID_HART_STATE_MANAGEMENT_EXTENSION, FID_HART_SUSPEND,
                 [suspend_type, resume_addr, opaque])
    }
}