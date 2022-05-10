mod trap;

use riscv::register::{scause::{self, Trap, Exception, Interrupt}, stval, stvec, sepc, sstatus};
use crate::syscall::syscall;
use crate::task::{RuntimeFlags, schedule};

pub use trap::{__enter_user_mode, __from_user_mode};
use crate::processor::{get_cur_task_context_in_this_hart};
use crate::plic;

pub fn init_stvec() {
    unsafe {
        stvec::write(__from_user_mode as usize, stvec::TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler() {
    let scause = scause::read();
    let stval = stval::read();
    let sepc = sepc::read();

    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            let context = get_cur_task_context_in_this_hart();
            context.sepc += 4;
            context.x[10] =
                syscall(context.x[17],
                        [context.x[10], context.x[11], context.x[12], context.x[13], context.x[14]]);
        },
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            schedule(RuntimeFlags::READY);
        },
        Trap::Interrupt(Interrupt::SupervisorExternal) => {
            plic::handle_interrupt();
        },
        _ => {
            info!("Unsupported trap {:?}, stval = {:#x}, sepc = {:#x}",scause.cause(), stval, sepc);
            match sstatus::read().spp() {
                sstatus::SPP::User => schedule(RuntimeFlags::ZOMBIE(1)),
                sstatus::SPP::Supervisor => panic!("Supervisor trap!"),
            };
        }
    }
}