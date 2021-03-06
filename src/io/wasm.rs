use wasm_bindgen::prelude::wasm_bindgen;

use core::{mem, ptr, cmp};
use core::fmt::{self, Write};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn info(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn debug(s: &str);
}

const BUFFER_CAPACITY: usize = 4096;

pub struct Console {
    fun: fn(&str),
    buffer: mem::MaybeUninit<[u8; BUFFER_CAPACITY]>,
    len: usize,
}

impl Console {
    fn new(fun: fn(&str)) -> Self {
        Self {
            fun,
            buffer: mem::MaybeUninit::uninit(),
            len: 0,
        }
    }

    #[inline(always)]
    fn as_ptr(&self) -> *const u8 {
        self.buffer.as_ptr() as _
    }

    #[inline(always)]
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.buffer.as_mut_ptr() as _
    }

    #[inline(always)]
    fn as_slice(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(self.as_ptr(), self.len)
        }
    }

    #[inline(always)]
    fn last_flush(&mut self) {
        if self.len > 0 {
            self.flush();
        }
    }

    fn flush(&mut self) {
        let text = unsafe {
            core::str::from_utf8_unchecked(self.as_slice())
        };
        (self.fun)(text);
        self.len = 0;
    }

    fn write_text(&mut self, text: &str) {
        //Yeah, how about to not write so much actually?
        debug_assert!(text.len() <= BUFFER_CAPACITY);

        if self.len + text.len() >= BUFFER_CAPACITY {
            self.flush();
        }

        let write_len = cmp::min(BUFFER_CAPACITY, text.len());
        unsafe {
            ptr::copy_nonoverlapping(text.as_ptr(), self.as_mut_ptr().add(self.len), write_len);
        }
        self.len += write_len;
    }
}

impl fmt::Write for Console {
    #[inline]
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.write_text(text);

        Ok(())
    }
}

impl crate::Logger {
    #[inline]
    ///Logger printer.
    pub(crate) fn print(record: &log::Record) {
        let mut console = match record.level() {
            log::Level::Trace => Console::new(debug),
            log::Level::Debug => Console::new(debug),
            log::Level::Info => Console::new(info),
            log::Level::Warn => Console::new(warn),
            log::Level::Error => Console::new(error),
        };

        let _ = write!(console, "{:<5} {{{}:{}}} - {}", record.level(), record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0), record.args());

        console.last_flush();
    }
}
