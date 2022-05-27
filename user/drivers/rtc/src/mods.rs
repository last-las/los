pub mod sysctl_mod {
    use core::arch::asm;

    use user_lib::syscall::dev_write;
    const SYSCTL_ADDRESS: usize = 0x5044_0000;
    const SOFT_RESET: usize = 0x30;
    const PERI_RESET: usize = 0x34;
    const CLK_EN_CENT: usize = 0x28;
    const APB1_CLK_EN: usize = 4;
    const CLK_EN_PERI: usize = 0x2c;
    const RTC_CLK_EN_PERI: usize = 29;
    use user_lib::syscall::*;

    fn read_sysctl_reg() -> u32 {
        dev_read_u32(SYSCTL_ADDRESS).unwrap() as u32
    }

    fn write_sysctl_reg(value: u32) {
        dev_write_u32(SYSCTL_ADDRESS, value).unwrap();
    }

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

    /// 读取时钟周期
    fn read_cycle() -> usize {
        sys_get_time() as usize
    }

    // read CPU current freq
    pub fn sync_clock_freq() {
        let freq = 1;
        let start_freq = read_cycle();
        while read_cycle() - start_freq < freq {
            continue;
        }
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
    // 时钟模式
    pub enum TimerMode {
        RtcTimerPause,   //< 0: Timer pause */
        RtcTimerRunning, //< 1: Timer time running */
        RtcTimerSetting, //< 2: Timer time setting */
        RtcTimerMax,     //< Max count of this enum*/
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
        pub time: RtcTime, /* time the alarm is set to */
    }

    impl RtcWkAlarm {
        pub fn new(
            year: usize,
            month: usize,
            day: usize,
            hour: usize,
            minute: usize,
            second: usize,
        ) -> Self {
            Self {
                time: RtcTime {
                    tm_sec: second,
                    tm_min: minute,
                    tm_hour: hour,
                    tm_mday: day,
                    tm_mon: month - 1,
                    tm_year: year - 1900,
                    tm_wday: get_weekday(year, month - 1, day),
                    tm_yday: rtc_month_days(month - 1, year) + day,
                    tm_isdst: -1,
                },
            }
        }

        pub fn default() -> Self {
            Self {
                time: RtcTime::default(),
            }
        }
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

pub mod register_ctrl_mod {
    use super::{
        rtc::{TimerMode, REG_REGISTER_CTRL, RTC_BASE_ADDRESS},
        sysctl_mod,
    };
    use user_lib::syscall::*;

    const REGISTER_CTRL_ADDTESS: usize = RTC_BASE_ADDRESS + REG_REGISTER_CTRL;

    pub fn rtc_timer_set_mode(mode: TimerMode) {
        match mode {
            TimerMode::RtcTimerPause => {
                write_read_enable(false);
                write_write_enable(false);
            }
            TimerMode::RtcTimerRunning => {
                write_read_enable(true);
                write_write_enable(false);
            }
            TimerMode::RtcTimerSetting => {
                write_read_enable(false);
                write_write_enable(true);
            }
            _ => {
                write_read_enable(false);
                write_write_enable(false);
            }
        }
        sysctl_mod::sync_clock_freq();
    }

    fn read_register_ctrl_reg() -> u32 {
        dev_read_u32(REGISTER_CTRL_ADDTESS).unwrap() as u32
    }

    fn write_register_ctrl_reg(value: u32) {
        dev_write_u32(REGISTER_CTRL_ADDTESS, value).unwrap();
    }

    pub fn write_read_enable(value: bool) {
        let v = read_register_ctrl_reg();
        let v = (v & !0x01) | ((value as u32) & 0x01);
        write_register_ctrl_reg(v);
    }

    pub fn write_write_enable(value: bool) {
        let v = read_register_ctrl_reg();
        let v = (v & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        write_register_ctrl_reg(v);
    }

    pub fn write_timer_mask(value: u8) {
        let v = read_register_ctrl_reg();
        let v = (v & !(0xff << 13)) | (((value as u32) & 0xff) << 13);
        write_register_ctrl_reg(v);
    }
    pub fn write_alarm_mask(value: u8) {
        let v = read_register_ctrl_reg();
        let v = (v & !(0xff << 21)) | (((value as u32) & 0xff) << 21);
        write_register_ctrl_reg(v);
    }

    pub fn write_initial_count_mask(value: bool) {
        let v = read_register_ctrl_reg();
        let v = (v & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        write_register_ctrl_reg(v);
    }

    pub fn interrupt_register_mask(value: bool) {
        let v = read_register_ctrl_reg();
        let v = (v & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        write_register_ctrl_reg(v);
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

    fn read_time_reg() -> u32 {
        dev_read_u32(TIME_ADDRESS).unwrap() as u32
    }

    fn write_time_reg(value: u32) {
        dev_write_u32(TIME_ADDRESS, value).unwrap();
    }

    fn read_hour() -> u8 {
        let v = read_time_reg();
        ((v >> 24) & 0x1f) as u8
    }
    fn write_hour(hour: u8) {
        let v = read_time_reg();
        let v = (v & !(0x1f << 24)) | (((hour as u32) & 0x1f) << 24);
        write_time_reg(v);
    }

    fn read_minute() -> u8 {
        let v = read_time_reg();
        ((v >> 16) & 0x3f) as u8
    }
    fn write_minute(value: u8) {
        let v = read_time_reg();
        let v = (v & &!(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        write_time_reg(v);
    }

    fn read_second() -> u8 {
        let v = read_time_reg();
        ((v >> 10) & 0x3f) as u8
    }
    fn write_second(value: u8) {
        let v = read_time_reg();
        let v = (v & &!(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        write_time_reg(v);
    }
    // 设置时间
    pub fn set_time(hour: usize, minute: usize, second: usize) {
        write_second(second as u8);
        write_minute(minute as u8);
        write_hour(hour as u8);
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

pub mod alarm_time_mod {
    use super::rtc::{REG_ALARM_TIME, RTC_BASE_ADDRESS};
    use user_lib::syscall::*;
    const TIME_ADDRESS: usize = RTC_BASE_ADDRESS + REG_ALARM_TIME;

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
        let v = (v & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        dev_write_u32(TIME_ADDRESS, v).unwrap();
    }
    // 设置时间
    pub fn set_alarm_time(hour: usize, minute: usize, second: usize) {
        write_second(second);
        write_minute(minute);
        write_hour(hour);
    }
    // 读取时间
    pub fn read_alarm_time() -> (u8, u8, u8) {
        (read_hour(), read_minute(), read_second())
    }
}

pub mod alarm_date_mod {
    use super::rtc::{REG_ALARM_DATE, RTC_BASE_ADDRESS};
    use user_lib::syscall::*;
    const ALARM_DATE_ADDRESS: usize = RTC_BASE_ADDRESS + REG_ALARM_DATE;

    fn read_alarm_date_reg() -> u32 {
        dev_read_u32(ALARM_DATE_ADDRESS).unwrap() as u32
    }

    fn write_alarm_date_reg(value: u32) {
        dev_write_u32(ALARM_DATE_ADDRESS, value).unwrap();
    }

    fn read_week() -> u8 {
        let v = read_alarm_date_reg();
        ((v >> 8) & 0x1f) as u8
    }
    fn write_week(value: u8) {
        let v = read_alarm_date_reg();
        let v = (v & !0x07) | ((value as u32) & 0x07);
        write_alarm_date_reg(v);
    }

    fn read_day() -> u8 {
        let v = read_alarm_date_reg();
        ((v >> 8) & 0x1f) as u8
    }
    fn write_day(value: u8) {
        let v = read_alarm_date_reg();
        let v = (v & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        write_alarm_date_reg(v);
    }

    fn read_month() -> u8 {
        let v = read_alarm_date_reg();
        ((v >> 16) & 0x0f) as u8
    }
    fn write_month(value: u8) {
        let v = read_alarm_date_reg();
        let v = (v & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        write_alarm_date_reg(v);
    }

    fn read_year() -> u16 {
        let v = read_alarm_date_reg();
        ((v >> 20) & 0x0fff) as u16
    }
    fn write_year(value: u16) {
        let v = read_alarm_date_reg();
        let v = (v & !(0x0fff << 20)) | (((value as u32) & 0x0fff) << 20);
        write_alarm_date_reg(v);
    }

    /// 设置alarm日期
    pub fn set_alarm_date(year: usize, month: usize, day: usize, week: usize) {
        write_year(year as u16);
        write_month(month as u8);
        write_day(day as u8);
        write_week(week as u8);
    }
    /// 获取闹钟日期
    pub fn read_alarm_date() -> (usize, usize, usize, usize) {
        (
            read_year() as usize,
            read_month() as usize,
            read_day() as usize,
            read_week() as usize,
        )
    }
}

pub mod interrupt_ctrl_mod {
    use super::rtc::{REG_INTERRUPT_CTRL, RTC_BASE_ADDRESS};
    use user_lib::syscall::*;

    const INTERRUPT_CTRL_ADDTESS: usize = RTC_BASE_ADDRESS + REG_INTERRUPT_CTRL;

    fn read_interrupt_ctrl_reg() -> u32 {
        dev_read_u32(INTERRUPT_CTRL_ADDTESS).unwrap() as u32
    }

    fn write_interrupt_ctrl_reg(value: u32) {
        dev_write_u32(INTERRUPT_CTRL_ADDTESS, value).unwrap();
    }

    /// Enable or disable RTC tick interrupt
    pub fn write_tick_enable(value: bool) {
        let v = read_interrupt_ctrl_reg();
        let v = (v & !0x01) | ((value as u32) & 0x01);
        write_interrupt_ctrl_reg(v);
    }

    /// Enable or disable RTC alarm interrupt
    fn write_alarm_enable(value: bool) {
        let v = read_interrupt_ctrl_reg();
        let v = (v & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        write_interrupt_ctrl_reg(v);
    }

    ///Set the interrupt mode of RTC tick interrupt
    /*
    RTC_INT_SECOND, /*!< 0: Interrupt every second */
    RTC_INT_MINUTE, /*!< 1: Interrupt every minute */
    RTC_INT_HOUR,   /*!< 2: Interrupt every hour */
    RTC_INT_DAY,    /*!< 3: Interrupt every day */
    RTC_INT_MAX     /*!< Max count of this enum*/
    */
    fn write_tick_interrupt_mode(value: u8) {
        let v = read_interrupt_ctrl_reg();
        let v = (v & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        write_interrupt_ctrl_reg(v);
    }

    /// Register callback of tick interrupt
    /// `mode`    Tick interrupt mode           \
    ///  0:second 1:minute    2:hour 3:day \
    pub fn rtc_tick_irq_register(mode: u8) {
        write_tick_enable(false);
        write_tick_interrupt_mode(mode);
        write_tick_enable(true);
    }

    /// Register callback of alarm interrupt
    ///*
    ///* @param   is_single_shot Indicates if single shot
    ///* @param   mask  The alarm compare mask for RTC alarm interrupt
    ///*   (rtc_mask_t) {
    ///*        .second = 1, Set this mask to compare Second
    ///*         .minute = 0, Set this mask to compare Minute
    ///*         .hour = 0,   Set this mask to compare Hour
    ///*         .week = 0,   Set this mask to compare Week
    ///*         .day = 0,    Set this mask to compare Day
    ///*         .month = 0,  Set this mask to compare Month
    ///*         .year = 0,   Set this mask to compare Year
    ///*   }
    ///* @param       callback  Callback of tick interrupt
    ///* @param       ctx       Param of callback
    ///* @param       priority  Priority of tick interrupt
    ///*
    pub fn rtc_alarm_irq_register(mode: u8) {
        write_tick_enable(false);
        write_alarm_enable(false);
        write_tick_interrupt_mode(mode);
        write_tick_enable(true);
        write_alarm_enable(true);
    }
}
