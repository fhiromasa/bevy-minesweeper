use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Sub},
};

use bevy::prelude::Component;

#[cfg(feature = "debug")]
use bevy_inspector_egui::{prelude::*, InspectorOptions};

#[cfg_attr(feature = "debug", derive(bevy::prelude::Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug", reflect(InspectorOptions))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinates {
    #[cfg_attr(feature = "debug", inspector(min = 0))]
    pub x: u16,
    #[cfg_attr(feature = "debug", inspector(min = 0))]
    pub y: u16,
}

impl Add<(i8, i8)> for Coordinates {
    type Output = Self;

    fn add(self, (x, y): (i8, i8)) -> Self::Output {
        Self {
            x: ((self.x as i16) + x as i16) as u16,
            y: ((self.y as i16) + y as i16) as u16,
        }
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
