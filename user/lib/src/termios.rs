use share::terminal::{Termios, TC_GET_ATTR, TC_SET_ATTR};
use share::ipc::{Msg, IOCTL, IOCTL_TYPE, ADDRESS, REPLY, REPLY_STATUS, PROC_NR, DEVICE};
use crate::syscall::{send, receive, getpid};
use share::syscall::error::SysError;

const TERMINAL_PID: usize = 1;

pub fn tc_get_attr(fd: usize) -> Result<Termios, SysError> {
    assert!(fd == 0 || fd == 1);
    let mut termios = Termios::empty();

    let mut message = Msg::empty();
    message.mtype = IOCTL;
    message.args[DEVICE] = 0;
    message.args[PROC_NR] = getpid();
    message.args[IOCTL_TYPE] = TC_GET_ATTR;
    message.args[ADDRESS] = &mut termios as *mut _ as usize;

    send(TERMINAL_PID, &message)?;
    receive(TERMINAL_PID as isize, &mut message)?;
    message.cvt_reply_message_to_result()?;

    Ok(termios)
}

pub fn tc_set_attr(fd: usize, termios: Termios) -> Result<(), SysError> {
    assert!(fd == 0 || fd == 1);
    let mut message = Msg::empty();
    message.mtype = IOCTL;
    message.args[DEVICE] = 0;
    message.args[PROC_NR] = getpid();
    message.args[IOCTL_TYPE] = TC_SET_ATTR;
    message.args[ADDRESS] = &termios as *const _ as usize;

    send(TERMINAL_PID, &message)?;
    receive(TERMINAL_PID as isize, &mut message)?;
    message.cvt_reply_message_to_result()?;

    Ok(())
}