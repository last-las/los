use self::rtc::{
    RtcTime, RtcWkAlarm, REG_ALARM_TIME, REG_INTERRUPT_CTRL, REG_REGISTER_CTRL, REG_TIME,
    RTC_BASE_ADDRESS,
};
use k210_pac::{
    rtc::{register_ctrl, RegisterBlock, REGISTER_CTRL},
    Peripherals, RTC,
};

pub mod sysctl_mod {
    use user_lib::syscall::dev_write;
    const SYSCTL_ADDRESS: usize = 0x5044_0000;
    const SOFT_RESET: usize = 0x30;
    const PERI_RESET: usize = 0x34;
    const CLK_EN_CENT: usize = 0x28;
    const APB1_CLK_EN: usize = 4;
    const CLK_EN_PERI: usize = 0x2c;
    const RTC_CLK_EN_PERI: usize = 29;
    use user_lib::syscall::*;
    /// reset rtc
    pub fn rtc_reset() {
        let peri = dev_read_u32(SYSCTL_ADDRESS + PERI_RESET).unwrap() as u32;
        let peri = (peri & !(0x01 << 29)) | (((1 as u32) & 0x01) << 29);
        dev_write_u32(SYSCTL_ADDRESS + PERI_RESET, peri);

        let peri = (peri & !(0x01 << 29)) | (((0 as u32) & 0x01) << 29);
        dev_write_u32(SYSCTL_ADDRESS + PERI_RESET, peri);
    }

    /// enable rtc
    pub fn rtc_enbale() {
        let clk_en_cent = dev_read_u32(SYSCTL_ADDRESS + CLK_EN_CENT).unwrap() as u32;
        let clk_en_cent = (clk_en_cent & !(0x01 << 4)) | (((1 as u32) & 0x01) << 4);
        dev_write_u32(SYSCTL_ADDRESS + CLK_EN_CENT, clk_en_cent).unwrap();

        let clk_en_peri = dev_read_u32(SYSCTL_ADDRESS + CLK_EN_PERI).unwrap() as u32;
        let clk_en_peri = (clk_en_peri & !(0x01 << 29)) | (((1 as u32) & 0x01) << 29);
        dev_write_u32(SYSCTL_ADDRESS + CLK_EN_PERI, clk_en_peri).unwrap();
    }
}

pub mod rtc {
    //? k210
    pub const RTC_BASE_ADDRESS: usize = 0x5046_0000;
    pub const REG_DATE: usize = 0x00;
    pub const REG_TIME: usize = 0x04;
    pub const REG_ALARM_DATE: usize = 0x08;
    pub const REG_ALARM_TIME: usize = 0x0c;
    pub const REG_INITIAL_COUNT: usize = 0x10;
    pub const REG_CURRENT_COUNT: usize = 0x14;
    pub const REG_INTERRUPT_CTRL: usize = 0x18;
    pub const REG_REGISTER_CTRL: usize = 0x1c;
    pub const REG_EXTENDED: usize = 0x28;

    pub const RTC_DAYS_IN_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    pub const RTC_YDAYS: [[usize; 13]; 2] = [
        /* Normal years */
        [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365],
        /* Leap years */
        [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366],
    ];
    /* Magic method to get weekday */
    pub fn get_weekday(year: usize, month: usize, day: usize) -> usize {
        let mut day = day + month;
        let mut year = year;
        let mut month = month;
        let mut weekday = 1;
        if day < 3 {
            year -= 1;
            weekday = year % 7;
        } else {
            year -= 2;
            weekday = (23 * month / 9 + day + 4 + year / 4 - year / 100 + year / 400) % 7;
        }
        weekday
    }

    //? time data
    pub struct RtcTime {
        pub tm_sec: usize,
        pub tm_min: usize,
        pub tm_hour: usize,
        pub tm_mday: usize,
        pub tm_mon: usize,
        pub tm_year: usize,
        pub tm_wday: usize, // 星期几
        pub tm_yday: usize,
        pub tm_isdst: isize,
    }

    impl RtcTime {
        pub fn new(
            year: usize,
            month: usize,
            day: usize,
            hour: usize,
            minute: usize,
            second: usize,
        ) -> Self {
            RtcTime {
                tm_sec: second,
                tm_min: minute,
                tm_hour: hour,
                tm_mday: day,
                tm_mon: month - 1,
                tm_year: year - 1900,
                tm_wday: get_weekday(year, month - 1, day),
                tm_yday: rtc_month_days(month - 1, year) + day,
                tm_isdst: -1,
            }
        }

        pub fn default() -> Self {
            RtcTime {
                tm_sec: 10,
                tm_min: 20,
                tm_hour: 10,
                tm_mday: 10,
                tm_mon: 10,
                tm_year: 2000,
                tm_wday: 10,
                tm_yday: 10,
                tm_isdst: 0,
            }
        }
    }

    //? wakeup alarm
    pub struct RtcWkAlarm {
        pub enabled: u8,   /* 0 = alarm disabled, 1 = alarm enabled */
        pub pending: u8,   /* 0 = alarm not pending, 1 = alarm pending */
        pub time: RtcTime, /* time the alarm is set to */
    }

    impl RtcWkAlarm {
        pub fn new() -> Self {
            RtcWkAlarm {
                enabled: 0,
                pending: 0,
                time: RtcTime::default(),
            }
        }

        pub fn from_time(&mut self, time: usize) {}

        pub fn to_time(&self) {}
    }

    pub fn leaps_thru_end_of(y: usize) -> usize {
        y / 4 - y / 100 + y / 400
    }

    pub fn rtc_month_days(month: usize, year: usize) -> usize {
        return (RTC_DAYS_IN_MONTH[month] as usize + ((is_leap_year(year) && month == 1) as usize))
            as usize;
    }

    pub fn is_leap_year(year: usize) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }

    pub fn register_ctrl_set(reg: usize, value: usize) {}
}

//todo
pub mod register_ctrl_mod {
    use user_lib::syscall::dev_read_u8;
    use user_lib::syscall::dev_write;

    use super::rtc::REG_REGISTER_CTRL;
    use super::rtc::RTC_BASE_ADDRESS;

    const READ_ENABLE: usize = 0;
    const WRITE_ENABLE: usize = 1;
    const TIMER_MASK: usize = 13;
    const ALARM_MASK: usize = 21;
    const INITIAL_COUNT_MASK: usize = 29;
    const INTERRUPT_REGISTER_MASK: usize = 30;

    pub fn read_enable(value: bool) {
        dev_write(
            RTC_BASE_ADDRESS + REG_REGISTER_CTRL + READ_ENABLE,
            ((value as u32) & 0x01) as usize,
            1,
        )
        .unwrap();
    }
    pub fn read_read_enable() -> usize {
        (dev_read_u8(RTC_BASE_ADDRESS + REG_REGISTER_CTRL + READ_ENABLE).unwrap() & 0x01) as usize
    }
    pub fn write_enable(value: bool) {
        dev_write(
            RTC_BASE_ADDRESS + REG_REGISTER_CTRL + WRITE_ENABLE,
            ((value as u32) & 0x01) as usize,
            1,
        )
        .unwrap();
    }
    pub fn read_write_enable() -> usize {
        (dev_read_u8(RTC_BASE_ADDRESS + REG_REGISTER_CTRL + WRITE_ENABLE).unwrap() & 0x01) as usize
    }
    pub fn initial_count_mask(value: bool) {
        dev_write(
            RTC_BASE_ADDRESS + REG_REGISTER_CTRL + INITIAL_COUNT_MASK,
            ((value as u32) & 0x01) as usize,
            1,
        )
        .unwrap();
    }
    pub fn read_initial_count_mask() -> usize {
        (dev_read_u8(RTC_BASE_ADDRESS + REG_REGISTER_CTRL + INITIAL_COUNT_MASK).unwrap() & 0x01)
            as usize
    }

    pub fn interrupt_register_mask(value: bool) {
        dev_write(
            RTC_BASE_ADDRESS + REG_REGISTER_CTRL + INTERRUPT_REGISTER_MASK,
            ((value as u32) & 0x01) as usize,
            1,
        )
        .unwrap();
    }

    pub fn read_interrupt_register_mask() -> usize {
        (dev_read_u8(RTC_BASE_ADDRESS + REG_REGISTER_CTRL + INTERRUPT_REGISTER_MASK).unwrap()
            & 0x01) as usize
    }

    pub fn alarm_mask(value: u8) {
        dev_write(
            RTC_BASE_ADDRESS + ALARM_MASK + REG_REGISTER_CTRL,
            (value & 0xff) as usize,
            1,
        )
        .unwrap();
    }

    pub fn read_alarm_mask() -> usize {
        dev_read_u8(RTC_BASE_ADDRESS + ALARM_MASK + REG_REGISTER_CTRL).unwrap() & 0xff
    }

    pub fn timer_mask(value: u8) {
        dev_write(
            RTC_BASE_ADDRESS + REG_REGISTER_CTRL + TIMER_MASK,
            (value & 0xff) as usize,
            1,
        )
        .unwrap();
    }
    pub fn read_time_mask() -> usize {
        dev_read_u8(RTC_BASE_ADDRESS + REG_REGISTER_CTRL + TIMER_MASK).unwrap() & 0xff
    }
}

pub mod initial_count_mod {
    use super::rtc::{REG_INITIAL_COUNT, RTC_BASE_ADDRESS};
    use user_lib::syscall::*;

    const INITIAL_COUNT_ADDTESS: usize = RTC_BASE_ADDRESS + REG_INITIAL_COUNT;

    fn read_initial_count_reg() -> u32 {
        dev_read_u32(INITIAL_COUNT_ADDTESS).unwrap() as u32
    }

    fn write_initial_count_reg(value: u32) {
        dev_write_u32(INITIAL_COUNT_ADDTESS, value).unwrap();
    }

    pub fn write_initial_count(count: u32) {
        let v = read_initial_count_reg();
        let v = (v & !0xffff_ffff) | ((count as u32) & 0xffff_ffff);
        write_initial_count_reg(v);
    }
}

pub mod current_count_mod {
    use super::rtc::{REG_CURRENT_COUNT, RTC_BASE_ADDRESS};
    use user_lib::syscall::*;

    const CURRENT_COUNT_ADDTESS: usize = RTC_BASE_ADDRESS + REG_CURRENT_COUNT;

    fn read_current_count_reg() -> u32 {
        dev_read_u32(CURRENT_COUNT_ADDTESS).unwrap() as u32
    }

    fn write_current_count_reg(value: u32) {
        dev_write_u32(CURRENT_COUNT_ADDTESS, value).unwrap();
    }

    pub fn write_current_count(count: u32) {
        let v = read_current_count_reg();
        let v = (v & !0xffff_ffff) | ((count as u32) & 0xffff_ffff);
        write_current_count_reg(v);
    }
}

pub mod time_mod {
    use super::rtc::{REG_TIME, RTC_BASE_ADDRESS};
    use user_lib::syscall::*;
    const TIME_ADDRESS: usize = RTC_BASE_ADDRESS + REG_TIME;
    // 初始化!
    pub fn reset_time() {
        dev_write_u32(TIME_ADDRESS, 0).unwrap();
    }

    fn read_hour() -> u8 {
        let v = dev_read_u32(TIME_ADDRESS).unwrap();
        ((v >> 24) & 0x1f) as u8
    }
    fn write_hour(hour: usize) {
        let v = dev_read_u32(TIME_ADDRESS).unwrap() as u32;
        let v = (v & !(0x1f << 24)) | (((hour as u32) & 0x1f) << 24);
        dev_write_u32(TIME_ADDRESS, v).unwrap();
    }

    fn read_minute() -> u8 {
        let v = dev_read_u32(TIME_ADDRESS).unwrap();
        ((v >> 16) & 0x3f) as u8
    }
    fn write_minute(value: usize) {
        let v = dev_read_u32(TIME_ADDRESS).unwrap() as u32;
        let v = (v & &!(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        dev_write_u32(TIME_ADDRESS, v).unwrap();
    }

    fn read_second() -> u8 {
        let v = dev_read_u32(TIME_ADDRESS).unwrap();
        ((v >> 10) & 0x3f) as u8
    }
    fn write_second(value: usize) {
        let v = dev_read_u32(TIME_ADDRESS).unwrap() as u32;
        let v = (v & &!(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        dev_write_u32(TIME_ADDRESS, v).unwrap();
    }
    // 设置时间
    pub fn set_time(hour: usize, minute: usize, second: usize) {
        write_second(second);
        write_minute(minute);
        write_hour(hour);
    }
    // 读取时间
    pub fn read_time() -> (u8, u8, u8) {
        (read_hour(), read_minute(), read_second())
    }
}

pub mod date_mod {

    use super::rtc::{REG_DATE, RTC_BASE_ADDRESS};
    use user_lib::syscall::*;

    const DATE_ADDRESS: usize = RTC_BASE_ADDRESS + REG_DATE;

    fn read_date_reg() -> u32 {
        dev_read_u32(DATE_ADDRESS).unwrap() as u32
    }

    fn write_date_reg(value: u32) {
        dev_write_u32(DATE_ADDRESS, value).unwrap();
    }

    fn read_week() -> u8 {
        let v = read_date_reg();
        (v & 0x07) as u8
    }
    fn write_week(value: u8) {
        let v = read_date_reg();
        let v = (v & !0x07) | ((value as u32) & 0x07);
        write_date_reg(v);
    }

    fn read_day() -> u8 {
        let v = read_date_reg();
        ((v >> 8) & 0x1f) as u8
    }
    fn write_day(value: u8) {
        let v = read_date_reg();
        let v = (v & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        write_date_reg(v);
    }

    fn read_month() -> u8 {
        let v = read_date_reg();
        ((v >> 16) & 0x0f) as u8
    }
    fn write_month(value: u8) {
        let v = read_date_reg();
        let v = (v & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        write_date_reg(v);
    }

    fn read_year() -> u16 {
        let v = read_date_reg();
        ((v >> 20) & 0x0fff) as u16
    }
    fn write_year(value: u16) {
        let v = read_date_reg();
        let v = (v & !(0x0fff << 20)) | (((value as u32) & 0x0fff) << 20);
        write_date_reg(v);
    }

    // 设置日期
    pub fn set_date(year: usize, month: usize, day: usize, week: usize) {
        write_year(year as u16);
        write_month(month as u8);
        write_day(day as u8);
        write_week(week as u8);
    }

    // 读取日期
    pub fn read_date() -> (usize, usize, usize, usize) {
        (
            read_year() as usize,
            read_month() as usize,
            read_day() as usize,
            read_week() as usize,
        )
    }
}

pub struct Rtc {
    pub time: RtcTime,
    pub alarm: RtcWkAlarm,
    pub rtc: RTC,
}

impl Rtc {
    pub fn new() -> Self {
        unsafe {
            let p = Peripherals::steal();
            Rtc {
                time: RtcTime::default(),
                alarm: RtcWkAlarm::new(),
                rtc: p.RTC,
            }
        }
    }
    /// 初始化
    pub fn init(&self) {
        sysctl_mod::rtc_reset();
        sysctl_mod::rtc_enbale();

        register_ctrl_mod::alarm_mask(0xff);
        register_ctrl_mod::timer_mask(0xff);
        register_ctrl_mod::initial_count_mask(true);
        register_ctrl_mod::interrupt_register_mask(true);

        initial_count_mod::write_initial_count(26000000);
        current_count_mod::write_current_count(1);

        register_ctrl_mod::read_enable(true);
        register_ctrl_mod::write_enable(true);

        time_mod::reset_time();
    }

    // 读取日期，返回年月日
    pub fn read_date(&self) -> (usize, usize, usize) {
        let tm = &self.time;

        (tm.tm_year, tm.tm_mon, tm.tm_mday)
    }

    // 读取时间，返回时分秒
    pub fn read_time(&self) -> (usize, usize, usize) {
        let tm = &self.time;

        (tm.tm_hour, tm.tm_min, tm.tm_sec)
    }

    // 设置rtc time的日期和时间
    pub fn set_time(
        &mut self,
        year: usize,
        month: usize,
        day: usize,
        hour: usize,
        minute: usize,
        second: usize,
    ) {
        let tm = RtcTime::new(year, month, day, hour, minute, second);
        time_mod::set_time(tm.tm_hour, tm.tm_min, tm.tm_sec);

        date_mod::set_date(tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday, tm.tm_wday);
        self.time = tm;
    }

    pub fn read_alarm(&mut self) {}

    pub fn set_alarm(&mut self, alrm: RtcWkAlarm) {}

    pub fn rtc_interrupt(&self) {}

    pub fn rtc_alarm_irq_enable(&self, enable: bool) {}
}
