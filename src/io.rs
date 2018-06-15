#[cfg(feature="color")]
extern crate termcolor;
extern crate log;


#[cfg(feature="timestamp")]
fn get_date() -> impl ::std::fmt::Display {
    extern crate chrono;
    chrono::offset::Local::now().format("%F %H:%M:%S%.3f %z")
}

#[cfg(feature="color")]
pub fn print(record: &log::Record) {
    use self::termcolor::{WriteColor, BufferWriter, ColorChoice, ColorSpec, Color};
    use std::io::Write;

    let writer = BufferWriter::stdout(ColorChoice::Auto);
    let mut buffer = writer.buffer();
    let level = record.level();
    let level_color = match level {
        log::Level::Error => Color::Red,
        log::Level::Warn => Color::Yellow,
        log::Level::Info => Color::Green,
        log::Level::Debug => Color::Cyan,
        log::Level::Trace => Color::Blue,
    };

    let _ = buffer.set_color(ColorSpec::new().set_fg(Some(level_color)));
    let _ = write!(&mut buffer, "{:<5} ", level);
    let _ = buffer.reset();

    #[cfg(feature="timestamp")]
    {
        let _ = write!(&mut buffer, "[{}] ", get_date());
    }

    let _ = buffer.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255, 255, 255))));
    let _ = write!(&mut buffer, "- {}", record.args());
    let _ = buffer.reset();

    let _ = write!(&mut buffer, " at {}:{}\n",
                   record.file().unwrap_or("UNKNOWN"),
                   record.line().unwrap_or(0));

    let _ = writer.print(&buffer);
}

#[cfg(not(feature="color"))]
pub fn print(record: &log::Record) {
    #[cfg(feature="timestamp")]
    {
        println!("{:<5} [{}] - {} at {}:{}",
                 record.level(),
                 get_date(),
                 record.args(),
                 record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0));

    }

    #[cfg(not(feature="timestamp"))]
    {
        println!("{:<5} - {} at {}:{}",
                 record.level(),
                 record.args(),
                 record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0));
    }
}
