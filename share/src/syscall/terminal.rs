bitflags! {
    pub struct Ciflag: u16 {
        const BRKINT = 0x0001;    /* signal interrupt on break */
        const ICRNL  = 0x0002;    /* map CR to NL on input */
        const IGNBRK = 0x0004;    /* ignore break */
        const IGNCR  = 0x0008;    /* ignore CR */
        const IGNPAR = 0x0010;    /* ignore characters with parity errors */
        const INLCR  = 0x0020;    /* map NL to CR on input */
        const INPCK  = 0x0040;    /* enable input parity check */
        const ISTRIP = 0x0080;    /* mask off 8th bit */
        const IXOFF  = 0x0100;    /* enable start/stop input control */
        const IXON   = 0x0200;    /* enable start/stop output control */
        const PARMRK = 0x0400;    /* mark parity errors in the input queue */
    }

    pub struct Coflag: u16 {}

    pub struct Ccflag: u16 {}

    pub struct Clflag: u16 {
        const ECHO	 = 0x0001;	/* enable echoing of input characters */
        const ECHOE	 = 0x0002;	/* echo ERASE as backspace */
        const ECHOK	 = 0x0004;	/* echo KILL */
        const ECHONL = 0x0008;	/* echo NL */
        const ICANON = 0x0010;	/* canonical input (erase and kill enabled) */
        const IEXTEN = 0x0020;	/* enable extended functions */
        const ISIG	 = 0x0040;	/* enable signals */
        const NOFLSH = 0x0080;  /* disable flush after interrupt or quit */
        const TOSTOP = 0x0100;	/* send SIGTTOU (job control, not implemented*/
    }
}

#[repr(C)]
pub struct Termios {
    pub c_iflag: Ciflag,
    pub c_oflag: Coflag,
    pub c_cflag: Ccflag,
    pub c_lflag: Clflag,
}

impl Termios {
    pub fn default() -> Self {
        Self {
            c_iflag: Ciflag::empty(),
            c_oflag: Coflag::empty(),
            c_cflag: Ccflag::empty(),
            c_lflag: Clflag::ECHO | Clflag::ECHOE | Clflag::ICANON,
        }
    }
}
