use bytesize::ByteSize;
use std::io;
use std::io::ErrorKind;
use std::str::FromStr;
use derive_setters::Setters;

/// Throttling parameters
///
/// Note: Throttle implements [`FromStr`] to read it from something like "10kiB,10MB"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Setters)]
pub struct Throttle {
    pub bandwidth: u32,
    pub burst: u32,
}

impl FromStr for Throttle {
    type Err = Box<io::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s
            .split(',')
            .map(|s| {
                ByteSize::from_str(s.trim())
                    .map_err(|err| io::Error::new(ErrorKind::InvalidInput, err))
            })
            .map(|b| -> io::Result<u32> {
                let byte_size = b?.as_u64();
                byte_size
                    .try_into()
                    .map_err(|err| io::Error::new(ErrorKind::InvalidInput, err))
            });

        let bandwidth = values
            .next()
            .transpose()?
            .ok_or_else(|| io::Error::new(ErrorKind::InvalidInput, "No bandwidth given."))?;

        let burst = values
            .next()
            .transpose()?
            .ok_or_else(|| io::Error::new(ErrorKind::InvalidInput, "No burst given."))?;

        Ok(Self { bandwidth, burst })
    }
}
