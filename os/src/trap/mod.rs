mod trap;

use riscv::register::{scause::{self, Trap, Exception, Interrupt}, stval, stvec, sepc};
use crate::syscall::syscall;
use crate::task::stop_current_and_run_next_task;

pub use trap::{__enter_user_mode, __from_user_mode, before_enter_user_mode};
use crate::processor::{get_cur_task_context_in_this_hart, get_hart_id};

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
                syscall(context.x[17], [context.x[10], context.x[11], context.x[12]])
                    as usize;
        },
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            stop_current_and_run_next_task();
        }
        _ => {
            panic!("hart{}: Unsupported trap {:?}, stval = {:#x}, sepc = {:#x}",
                   get_hart_id(), scause.cause(), stval, sepc);
        }
    }
}