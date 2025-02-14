use std::default::Default;
use std::convert::{TryFrom, Into};
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum Scale {
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine
}

impl Default for Scale {
    fn default() -> Scale {
        Scale::Kelvin
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Temperature {
    kelvin: f32,
    scale: Scale
}

pub const ABSOLUTE_ZERO:           Temperature = Temperature { kelvin:   0.0,    scale: Scale::Kelvin };
pub const FREEZING_POINT_OF_BRINE: Temperature = Temperature { kelvin: 255.37,   scale: Scale::Kelvin };
pub const FREEZING_POINT_OF_WATER: Temperature = Temperature { kelvin: 273.15,   scale: Scale::Kelvin };
pub const BOILING_POINT_OF_WATER:  Temperature = Temperature { kelvin: 373.1339, scale: Scale::Kelvin };

/// This is the lowest temperature we can represent.
pub const MIN: Temperature = ABSOLUTE_ZERO;
/// This is the highest temperature we can represent.
pub const MAX: Temperature = Temperature { kelvin: std::f32::MAX, scale: Scale::Kelvin };

impl Temperature {
    pub fn new(kelvin: f32) -> Temperature {
        Temperature::kelvin(kelvin)
    }

    pub fn celsius(celsius: f32) -> Temperature {
        Temperature {
            kelvin: celsius + 273.15,
            scale: Scale::Celsius
        }
    }

    pub fn fahrenheit(fahrenheit: f32) -> Temperature {
        Temperature {
            kelvin: (fahrenheit + 459.67) * (5.0/9.0),
            scale: Scale::Fahrenheit
        }
    }

    pub fn kelvin(kelvin: f32) -> Temperature {
        Temperature {
            kelvin: kelvin,
            scale: Scale::Kelvin
        }
    }

    pub fn rankine(rankine: f32) -> Temperature {
        Temperature {
            kelvin: (rankine + 459.67),
            scale: Scale::Rankine
        }
    }

    #[allow(non_snake_case)]
    pub fn C(celsius: f32) -> Temperature {
        Temperature::celsius(celsius)
    }

    #[allow(non_snake_case)]
    pub fn F(fahrenheit: f32) -> Temperature {
        Temperature::fahrenheit(fahrenheit)
    }

    #[allow(non_snake_case)]
    pub fn K(kelvin: f32) -> Temperature {
        Temperature::kelvin(kelvin)
    }

    #[allow(non_snake_case)]
    pub fn R(rankine: f32) -> Temperature {
        Temperature::rankine(rankine)
    }

    #[allow(non_snake_case)]
    pub fn C_range(celsius_range: std::ops::Range<f32>) -> std::ops::Range<Temperature> {
        Temperature::C(celsius_range.start) .. Temperature::C(celsius_range.end)
    }

    #[allow(non_snake_case)]
    pub fn F_range(fahrenheit_range: std::ops::Range<f32>) -> std::ops::Range<Temperature> {
        Temperature::F(fahrenheit_range.start) .. Temperature::F(fahrenheit_range.end)
    }

    #[allow(non_snake_case)]
    pub fn K_range(kelvin_range: std::ops::Range<f32>) -> std::ops::Range<Temperature> {
        Temperature::K(kelvin_range.start) .. Temperature::K(kelvin_range.end)
    }

    #[allow(non_snake_case)]
    pub fn R_range(rankine_range: std::ops::Range<f32>) -> std::ops::Range<Temperature> {
        Temperature::R(rankine_range.start) .. Temperature::R(rankine_range.end)
    }

    pub fn to_celsius(self) -> Temperature {
        Temperature {
            kelvin: self.kelvin,
            scale: Scale::Celsius
        }
    }

    pub fn to_fahrenheit(self) -> Temperature {
        Temperature {
            kelvin: self.kelvin,
            scale: Scale::Fahrenheit
        }
    }

    pub fn to_kelvin(self) -> Temperature {
        Temperature {
            kelvin: self.kelvin,
            scale: Scale::Kelvin
        }
    }

    pub fn to_rankine(self) -> Temperature {
        Temperature {
            kelvin: self.kelvin,
            scale: Scale::Rankine
        }
    }
}

impl TryFrom<f32> for Temperature {
    type Error = ();
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value >= 0.0 {
            Ok(Temperature::kelvin(value))
        } else {
            Err(())
        }
    }
}

impl Into<f32> for Temperature {
    fn into(self) -> f32 {
        match self.scale {
            Scale::Celsius    => self.kelvin - 273.15,
            Scale::Fahrenheit => self.kelvin * (9.0/5.0) - 459.67,
            Scale::Kelvin     => self.kelvin,
            Scale::Rankine    => self.kelvin * (9.0/5.0)
        }
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: f32 = Temperature::into(*self);
        match self.scale {
            Scale::Celsius    => write!(f, "{} °C", value),
            Scale::Fahrenheit => write!(f, "{} °F", value),
            Scale::Kelvin     => write!(f, "{} K",  value),
            Scale::Rankine    => write!(f, "{} °R", value)
        }
    }
}
