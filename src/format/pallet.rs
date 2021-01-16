use super::Color;
use log::Level;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pallet {
    pub error: Color,
    pub warn: Color,
    pub info: Color,
    pub debug: Color,
    pub trace: Color,
}

impl Pallet {
    pub fn select(&self, level: Level) -> Color {
        match level {
            Level::Error => self.error,
            Level::Warn => self.warn,
            Level::Info => self.info,
            Level::Debug => self.debug,
            Level::Trace => self.trace,
        }
    }
}
