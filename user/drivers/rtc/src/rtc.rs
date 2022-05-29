// use k210_pac::{Peripherals, RTC};

use k210::sysctl::{self, clock, clock_enable, clock_get_freq, reset};

use crate::mods::rtc::*;
use crate::mods::*;

pub struct Rtc {
    pub time: RtcTime,
    pub alarm: RtcWkAlarm,
    // pub rtc: RTC,
}

impl Rtc {
    pub fn new() -> Self {
        unsafe {
            // let p = Peripherals::steal();
            Rtc {
                time: RtcTime::default(),
                alarm: RtcWkAlarm::default(),
                // rtc: p.RTC,
            }
        }
    }
    /// 初始化
    pub fn init(&self) {
        // sysctl_mod::rtc_reset();
        reset(reset::RTC);
        // sysctl_mod::rtc_enbale();
        clock_enable(clock::RTC);

        register_ctrl_mod::write_alarm_mask(0xfe);
        register_ctrl_mod::write_timer_mask(0xfe);
        register_ctrl_mod::write_initial_count_mask(true);
        register_ctrl_mod::interrupt_register_mask(true);

        // initial_count_mod::write_initial_count(clock_get_freq(clock::IN0));
        self.timer_set_clock_frequency(clock_get_freq(clock::IN0))
            .timer_set_clock_count_value(1)
            .set_mode(TimerMode::RtcTimerRunning);
        // current_count_mod::write_current_count(1);
        // register_ctrl_mod::rtc_timer_set_mode(TimerMode::RtcTimerRunning);
    }

    fn set_mode(&self, mode: TimerMode) -> &Self {
        register_ctrl_mod::rtc_timer_set_mode(mode);
        &self
    }

    fn timer_set_clock_frequency(&self, clock_freq: u32) -> &Self {
        initial_count_mod::write_initial_count(clock_freq);
        &self
    }

    fn timer_set_clock_count_value(&self, count: u32) -> &Self {
        self.set_mode(TimerMode::RtcTimerSetting);
        current_count_mod::write_current_count(count);
        self.set_mode(TimerMode::RtcTimerRunning);
        &self
    }

    // 读取日期，返回年月日
    pub fn read_date(&self) -> (usize, usize, usize) {
        let tm = &self.time;

        (tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday)
    }

    // 读取时间，返回时分秒
    pub fn read_time(&self) -> (usize, usize, usize) {
        let tm = &self.time;

        (tm.tm_hour, tm.tm_min, tm.tm_sec)
    }

    // 设置rtc time的日期和时间
    pub fn timer_set(
        &mut self,
        year: usize,
        month: usize,
        day: usize,
        hour: usize,
        minute: usize,
        second: usize,
    ) {
        let tm = RtcTime::new(year, month, day, hour, minute, second);
        self.set_mode(TimerMode::RtcTimerSetting);
        time_mod::set_time(tm.tm_hour, tm.tm_min, tm.tm_sec);

        date_mod::set_date(tm.tm_year, tm.tm_mon, tm.tm_mday, tm.tm_wday);
        self.time = tm;
        self.set_mode(TimerMode::RtcTimerRunning);
    }

    // 读取闹钟日期
    pub fn read_alarm_date(&self) -> (usize, usize, usize) {
        let tm = &self.alarm.time;

        (tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday)
    }
    // 读取闹钟时间
    pub fn read_alarm_time(&mut self) -> (usize, usize, usize) {
        let tm = &self.alarm.time;

        (tm.tm_hour, tm.tm_min, tm.tm_sec)
    }

    // 设置alarm
    pub fn set_alarm(
        &mut self,
        year: usize,
        month: usize,
        day: usize,
        hour: usize,
        minute: usize,
        second: usize,
    ) {
        let tm = RtcWkAlarm::new(year, month, day, hour, minute, second);
        alarm_time_mod::set_alarm_time(tm.time.tm_hour, tm.time.tm_min, tm.time.tm_sec);
        alarm_date_mod::set_alarm_date(
            tm.time.tm_year,
            tm.time.tm_mon,
            tm.time.tm_mday,
            tm.time.tm_wday,
        );

        self.alarm = tm;
    }

    pub fn irq_register(&self, mode: u8) {
        interrupt_ctrl_mod::rtc_tick_irq_register(mode);
    }

    pub fn tick_interrupt_enable(&self, enable: bool) {
        interrupt_ctrl_mod::write_tick_enable(enable);
    }
}
