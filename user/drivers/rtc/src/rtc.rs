use k210::sysctl::{self, clock, clock_enable, clock_get_freq, reset};

use crate::mods::rtc::*;
use crate::mods::*;

pub struct Rtc {
    pub time: RtcTime,
    pub alarm: RtcWkAlarm,
}

impl Rtc {
    pub fn new() -> Self {
        Rtc {
            time: RtcTime::default(),
            alarm: RtcWkAlarm::default(),
        }
    }
    /// 初始化
    pub fn init(&mut self) {
        reset(reset::RTC);
        clock_enable(clock::RTC);

        register_ctrl_mod::write_alarm_mask(0xfe);
        register_ctrl_mod::write_timer_mask(0xfe);
        register_ctrl_mod::write_initial_count_mask(true);
        register_ctrl_mod::interrupt_register_mask(true);

        self.timer_set_clock_frequency(clock_get_freq(clock::IN0))
            .timer_set_clock_count_value(1)
            .set_mode(TimerMode::RtcTimerRunning);
    }

    fn set_mode(&mut self, mode: TimerMode) -> &mut Self {
        register_ctrl_mod::rtc_timer_set_mode(mode);
        self
    }

    fn timer_set_clock_frequency(&mut self, clock_freq: u32) -> &mut Self {
        initial_count_mod::write_initial_count(clock_freq);
        self
    }

    fn timer_set_clock_count_value(&mut self, count: u32) -> &mut Self {
        self.set_mode(TimerMode::RtcTimerSetting);
        current_count_mod::write_current_count(count);
        self.set_mode(TimerMode::RtcTimerRunning)
    }
    // 更新时钟
    fn update_rtc_time(&mut self, tm: RtcTime) -> &mut Self {
        self.time = tm;
        self
    }

    // 读取日期，返回年月日
    pub fn read_date(&mut self) -> (usize, usize, usize) {
        let tm = &self.time;

        (tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday)
    }

    // 读取时间，返回时分秒
    pub fn read_time(&mut self) -> (usize, usize, usize) {
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
    ) -> &mut Self {
        let tm = RtcTime::new(year, month, day, hour, minute, second);
        self.timer_set_time(tm.tm_hour, tm.tm_min, tm.tm_sec)
            .timer_set_date(tm.tm_year, tm.tm_mon, tm.tm_mday, tm.tm_wday)
            .update_rtc_time(tm)
    }
    // 设置时钟的时间
    fn timer_set_time(&mut self, hour: usize, minute: usize, second: usize) -> &mut Self {
        self.set_mode(TimerMode::RtcTimerSetting);
        let mut hour = hour;
        let mut minute = minute;
        let mut second = second;
        if second >= 60 {
            minute += 1;
            second %= 60;
        }
        if minute >= 60 {
            hour += 1;
            minute %= 60;
        }
        time_mod::set_time(hour, minute, second);
        self.time.update_time(hour, minute, second);
        self.set_mode(TimerMode::RtcTimerRunning)
    }
    // 设置时钟的日期
    fn timer_set_date(&mut self, year: usize, month: usize, day: usize, week: usize) -> &mut Self {
        self.set_mode(TimerMode::RtcTimerSetting);
        date_mod::set_date(year, month, day, week);
        self.set_mode(TimerMode::RtcTimerRunning)
    }

    // 更新时钟
    pub fn timer_update(&mut self) {
        match self.read_tick_interrupt_mode() {
            0 => {
                // 秒中断
                let (h, m, s) = self.read_time();
                self.timer_set_time(h, m, s + 1);
            }
            1 => {
                // 分钟中断
                let (h, m, s) = self.read_time();
                self.timer_set_time(h, m + 1, s);
            }
            _ => {}
        };
    }

    // 读取闹钟日期
    pub fn read_alarm_date(&mut self) -> (usize, usize, usize) {
        let tm = &mut self.alarm.time;

        (tm.tm_year + 1900, tm.tm_mon + 1, tm.tm_mday)
    }
    // 读取闹钟时间
    pub fn read_alarm_time(&mut self) -> (usize, usize, usize) {
        let tm = &mut self.alarm.time;

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

    // 注册中断
    pub fn irq_register(&mut self, mode: u8) {
        self.tick_set_interrupt(false);
        self.tick_set_interrupt_mode(mode);
        self.tick_set_interrupt(true);
    }

    // 读取时钟中断类型
    fn read_tick_interrupt_mode(&mut self) -> u8 {
        interrupt_ctrl_mod::read_tick_interrupt_mode()
    }

    // 设置时钟中断类型
    fn tick_set_interrupt_mode(&mut self, mode: u8) -> &mut Self {
        self.tick_set_interrupt(false);
        self.set_mode(TimerMode::RtcTimerSetting);
        interrupt_ctrl_mod::write_tick_interrupt_mode(mode);
        self.set_mode(TimerMode::RtcTimerRunning);
        self.tick_set_interrupt(true)
    }

    // 设置时钟计时中断
    pub fn tick_set_interrupt(&mut self, enable: bool) -> &mut Self {
        self.set_mode(TimerMode::RtcTimerSetting);
        interrupt_ctrl_mod::write_tick_enable(enable);
        self.set_mode(TimerMode::RtcTimerRunning)
    }
}
