use core::fmt::{self, Debug};

/// Cardinal directions. Each one matches one of the user LEDs.
pub enum CompassDirection {
    /// North / LD3
    North,
    /// Northeast / LD5
    NorthEast,
    /// East / LD7
    East,
    /// Southeast / LD9
    SouthEast,
    /// South / LD10
    South,
    /// Southwest / LD8
    SouthWest,
    /// West / LD6
    West,
    /// Northwest / LD4
    NorthWest,
}

impl Debug for CompassDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::North => write!(f, "North"),
            Self::NorthWest => write!(f, "NorthWest"),
            Self::NorthEast => write!(f, "NorthEast"),
            Self::South => write!(f, "South"),
            Self::SouthWest => write!(f, "SouthWest"),
            Self::SouthEast => write!(f, "SouthEast"),
            Self::West => write!(f, "West"),
            Self::East => write!(f, "East"),
        }
    }
}
