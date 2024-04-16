use std::{
    result::Result as StdResult,
    sync::{Mutex, MutexGuard, PoisonError},
};

use once_cell::sync::OnceCell;
use serde::{
    de::{self, Deserializer},
    Deserialize, Serialize,
};
use strum::{Display, EnumString};

pub use self::_printer::{eprinter, printer, Printer};

static GLOBAL_PRINTER: OnceCell<Mutex<self::_printer::Printer>> = OnceCell::new();
static GLOBAL_EPRINTER: OnceCell<Mutex<self::_printer::Printer>> = OnceCell::new();

#[derive(EnumString, Display, Deserialize, Copy, Clone, Debug, PartialEq, Eq, clap::ValueEnum)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
#[derive(Default)]
pub enum OutFormat {
    Raw,
    #[default]
    Yaml,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, clap::ValueEnum, Display, EnumString)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
#[derive(Default)]
pub enum ColorChoice {
    Always,
    Ansi,
    #[default]
    Auto,
    Never,
}

impl<'de> Deserialize<'de> for ColorChoice {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> StdResult<Self, D::Error> {
        use std::str::FromStr;
        let s = <String>::deserialize(deserializer)?;
        ColorChoice::from_str(&s).map_err(de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum Color {
    Black,
    Blue,
    Green,
    Red,
    Cyan,
    Magenta,
    Yellow,
    White,
}

#[cfg(feature = "color")]
mod _printer {
    use std::io::{self, IsTerminal};

    use termcolor::{
        Color as TermColorColor, ColorChoice as TermColorChoice, ColorSpec, StandardStream,
        WriteColor,
    };

    use super::*;

    fn is_tty<S: IsTerminal>(stream: S) -> TermColorChoice {
        if stream.is_terminal() {
            TermColorChoice::Auto
        } else {
            TermColorChoice::Never
        }
    }

    #[allow(missing_debug_implementations)]
    pub struct Printer(StandardStream);

    pub fn printer() -> MutexGuard<'static, Printer> {
        GLOBAL_PRINTER
            .get_or_init(|| Mutex::new(Printer(StandardStream::stdout(TermColorChoice::Auto))))
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    pub fn eprinter() -> MutexGuard<'static, Printer> {
        GLOBAL_EPRINTER
            .get_or_init(|| Mutex::new(Printer(StandardStream::stderr(TermColorChoice::Auto))))
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    impl Printer {
        pub fn init(color: ColorChoice) {
            use TermColorChoice::*;
            let (choice, echoice) = match color {
                ColorChoice::Always => (Always, Always),
                ColorChoice::Auto => (is_tty(io::stdin()), is_tty(io::stdout())),
                ColorChoice::Ansi => (AlwaysAnsi, AlwaysAnsi),
                ColorChoice::Never => (Never, Never),
            };

            printer().set_stream(StandardStream::stdout(choice));
            eprinter().set_stream(StandardStream::stderr(echoice));
        }

        fn set_stream(&mut self, stream: StandardStream) { self.0 = stream; }

        pub fn set_color(&mut self, color: Color) {
            let _ = self
                .0
                .set_color(ColorSpec::new().set_fg(Some(color.into_termcolor())));
        }

        pub fn reset(&mut self) { let _ = self.0.reset(); }
    }

    impl io::Write for Printer {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> { self.0.write(buf) }

        fn flush(&mut self) -> io::Result<()> { self.0.flush() }
    }

    impl Color {
        fn into_termcolor(self) -> TermColorColor {
            match self {
                Color::Black => TermColorColor::Black,
                Color::Blue => TermColorColor::Blue,
                Color::Green => TermColorColor::Green,
                Color::Red => TermColorColor::Red,
                Color::Cyan => TermColorColor::Cyan,
                Color::Magenta => TermColorColor::Magenta,
                Color::Yellow => TermColorColor::Yellow,
                Color::White => TermColorColor::White,
            }
        }
    }
}

#[cfg(not(feature = "color"))]
mod _printer {
    use std::io;

    use super::*;

    enum StandardStream {
        Stdout(io::Stdout),
        Stderr(io::Stderr),
    }

    #[allow(missing_debug_implementations)]
    pub struct Printer(StandardStream);

    pub fn printer() -> MutexGuard<'static, Printer> {
        GLOBAL_PRINTER
            .get_or_init(|| Mutex::new(Printer(StandardStream::Stdout(io::stdout()))))
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    pub fn eprinter() -> MutexGuard<'static, Printer> {
        GLOBAL_EPRINTER
            .get_or_init(|| Mutex::new(Printer(StandardStream::Stderr(io::stderr()))))
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    impl Printer {
        pub fn init(_color: ColorChoice) {
            let _a = printer();
            let _a = eprinter();
        }

        pub fn set_color(&mut self, _color: Color) {}

        pub fn reset(&mut self) {}
    }

    impl io::Write for Printer {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            match self.0 {
                StandardStream::Stdout(ref mut s) => s.write(buf),
                StandardStream::Stderr(ref mut s) => s.write(buf),
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            match self.0 {
                StandardStream::Stdout(ref mut s) => s.flush(),
                StandardStream::Stderr(ref mut s) => s.flush(),
            }
        }
    }
}
