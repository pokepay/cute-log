#[cfg(feature="timestamp")]
#[inline(always)]
fn get_date() -> impl core::fmt::Display {
    struct TimeDate(time::PrimitiveDateTime);

    impl core::fmt::Display for TimeDate {
        #[inline(always)]
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}-{:02}-{:02} {:02}:{:02}:{:02}", self.0.year(), self.0.month(), self.0.day(), self.0.hour(), self.0.minute(), self.0.second())
        }
    }

    TimeDate(std::time::SystemTime::now().into())
}

impl crate::Logger {
    #[inline]
    ///Logger printer.
    pub(crate) fn print(record: &log::Record) {
        #[cfg(feature="timestamp")]
        {
            println!("{:<5} [{}] {{{}:{}}} - {}",
                     record.level(),
                     get_date(),
                     record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0),
                     record.args());

        }

        #[cfg(not(feature="timestamp"))]
        {
            println!("{:<5} {{{}:{}}} - {}",
                     record.level(),
                     record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0),
                     record.args());
        }
    }
}
