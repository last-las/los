use k210_pac::Peripherals;

use crate::mods::rtc::*;
use crate::mods::*;

pub struct Rtc {
    pub time: RtcTime,
    pub alarm: RtcWkAlarm,
}

impl Rtc {
    pub fn new() -> Self {
        unsafe {
            let p = Peripherals::steal();
            Rtc {
                time: RtcTime::default(),
                alarm: RtcWkAlarm::default(),
            }
        }
    }
    /// 初始化
    pub fn init(&self) {
        sysctl_mod::rtc_reset();
        sysctl_mod::rtc_enbale();

        register_ctrl_mod::write_alarm_mask(0xff);
        register_ctrl_mod::write_timer_mask(0xff);
        register_ctrl_mod::write_initial_count_mask(true);
        register_ctrl_mod::interrupt_register_mask(true);

        initial_count_mod::write_initial_count(26000000);
        current_count_mod::write_current_count(1);

        register_ctrl_mod::rtc_timer_set_mode(TimerMode::RtcTimerRunning);
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
            tm.time.tm_year + 1900,
            tm.time.tm_mon + 1,
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
