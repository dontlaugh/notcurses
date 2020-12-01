//! Handy [`NcCell`] constructors

pub use crate::{NcCell, NcChannels, NcCharBackstop, NcStyleMask};

impl NcCell {
    /// [`NcCell`] constructor expecting [`char`], [`NcStyleMask`] and [`NcChannels`]
    #[inline]
    pub const fn new(ch: char, stylemask: NcStyleMask, channels: NcChannels) -> Self {
        NcCell {
            gcluster: ch as u32,
            gcluster_backstop: 0 as NcCharBackstop,
            reserved: 0,
            stylemask,
            channels,
        }
    }

    /// [`NcCell`] simple constructor just expecting a [`char`]
    #[inline]
    pub const fn with_char(ch: char) -> Self {
        Self::new(ch, 0 as NcStyleMask, 0 as NcChannels)
    }

    /// [`NcCell`] simple constructor for an empty cell
    #[inline]
    pub const fn new_blank() -> Self {
        Self::with_char(0 as char)
    }
}
