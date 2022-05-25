use k210_pac::{
    rtc::{register_ctrl, RegisterBlock, REGISTER_CTRL},
    Peripherals, RTC,
};
use user_lib::syscall::{dev_read_u32, dev_write_u32};

use self::rtc::{
    RtcTime, RtcWkAlarm, NSEC_PER_SEC, REG_ALARM_TIME, REG_INTERRUPT_CTRL, REG_REGISTER_CTRL,
    REG_TIME, RTC_BASE_ADDRESS,
};

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

    //? Parameters used to convert the timespec values
    pub const MSEC_PER_SEC: usize = 1000;
    pub const USEC_PER_MSEC: usize = 1000;
    pub const NSEC_PER_USEC: usize = 1000;
    pub const NSEC_PER_MSEC: usize = 1000000;
    pub const USEC_PER_SEC: usize = 1000000;
    pub const NSEC_PER_SEC: usize = 1000000000;
    pub const FSEC_PER_SEC: usize = 1000000000000000;

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
                tm_wday: get_weekday(year, month, day),
                tm_yday: rtc_month_days(month, year) + day,
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
        write_reg(REG_REGISTER_CTRL, 0xffffff);
        // unsafe {
        //     self.rtc.register_ctrl.write(|f| {
        //         f.alarm_mask()
        //             .bits(0xff)
        //             .timer_mask()
        //             .bits(0xff)
        //             .initial_count_mask()
        //             .set_bit()
        //             .interrupt_register_mask()
        //             .set_bit()
        //     });

        //     self.rtc.initial_count.write(|f| f.count().bits(26000000));

        //     self.rtc.current_count.write(|f| f.count().bits(1));

        //     self.rtc
        //         .register_ctrl
        //         .write(|f| f.read_enable().set_bit().write_enable().set_bit())
        // }
    }

    // 读取时间，返回年月日
    pub fn read_time(&self) -> (usize, usize, usize) {
        let tm = &self.time;

        (tm.tm_year, tm.tm_mon, tm.tm_mday)
    }

    // 设置rtc time的时间
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
        unsafe {
            self.rtc.time.write(|f| {
                f.hour()
                    .bits(hour as u8)
                    .minute()
                    .bits(minute as u8)
                    .second()
                    .bits(second as u8)
            });
            self.rtc.date.write(|f| {
                f.year()
                    .bits((tm.tm_year + 1900) as u16)
                    .week()
                    .bits(tm.tm_wday as u8)
                    .month()
                    .bits((tm.tm_mon + 1) as u8)
                    .day()
                    .bits(tm.tm_mday as u8)
            })
        }
        self.time = tm;
    }

    pub fn read_alarm(&mut self) {}

    pub fn set_alarm(&mut self, alrm: RtcWkAlarm) {}

    pub fn rtc_interrupt(&self) {}

    pub fn rtc_alarm_irq_enable(&self, enable: bool) {}
}

fn do_div(n: usize, base: usize) -> usize {
    n / base
}

fn write_reg(reg: usize, dword: u32) {
    dev_write_u32(RTC_BASE_ADDRESS + reg, dword).unwrap();
}

fn read_reg(reg: usize) -> u32 {
    dev_read_u32(RTC_BASE_ADDRESS + reg).unwrap() as u32
}
