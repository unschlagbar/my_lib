use crate::primitives::Vec2;



#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum UiSize {
    Fill(),
    Undefined(),
    Auto(),
    AutoMin(UIUnit),
    AutoMax(UIUnit),
    AutoMinMax(UIUnit, UIUnit),
    Size(UIUnit),
    SizeMin(UIUnit, UIUnit),
    SizeMax(UIUnit, UIUnit),
    SizeMinMax(UIUnit, UIUnit, UIUnit),
    Sub(UIUnit, UIUnit),
    Add(UIUnit, UIUnit),

}

impl UiSize {
    #[inline]
    pub fn width(&self, parent_size: Vec2) -> f32 {
        match self {
            Self::Fill() => {
                parent_size.x
            },
            Self::Undefined() => {
                0.0
            },
            Self::Auto() => {
                100.0
            },
            Self::AutoMin(min) => {
                100f32.max(min.pixelx(parent_size))
            },
            Self::AutoMax(max) => {
                100f32.min(max.pixelx(parent_size))
            },
            Self::AutoMinMax(min, max) => {
                100f32.clamp(min.pixelx(parent_size), max.pixelx(parent_size))
            },
            Self::Size(unit) => {
                unit.pixelx(parent_size)
            },
            Self::SizeMin(unit, min) => {
                unit.pixelx(parent_size).max(min.pixelx(parent_size))
            },
            Self::SizeMax(unit, max) => {
                unit.pixelx(parent_size).min(max.pixelx(parent_size))
            },
            Self::SizeMinMax(unit, min, max) => {
                unit.pixelx(parent_size).clamp(min.pixelx(parent_size), max.pixelx(parent_size))
            },
            Self::Add(first, second) => {
                first.pixelx(parent_size) + second.pixelx(parent_size)
            },
            Self::Sub(first, second) => {
                (first.pixelx(parent_size) - second.pixelx(parent_size)).max(0.0)
            },
        }
    }

    #[inline]
    pub fn height(&self, parent_size: Vec2) -> f32 {
        match self {
            Self::Fill() => {
                parent_size.y
            },
            Self::Undefined() => {
                0.0
            },
            Self::Auto() => {
                50.0
            },
            Self::AutoMin(min) => {
                50f32.max(min.pixely(parent_size))
            },
            Self::AutoMax(max) => {
                50f32.min(max.pixely(parent_size))
            },
            Self::AutoMinMax(min, max) => {
                50f32.clamp(min.pixely(parent_size), max.pixely(parent_size))
            },
            Self::Size(unit) => {
                unit.pixely(parent_size)
            },
            Self::SizeMin(unit, min) => {
                unit.pixely(parent_size).max(min.pixely(parent_size))
            },
            Self::SizeMax(unit, max) => {
                unit.pixely(parent_size).min(max.pixely(parent_size))
            },
            Self::SizeMinMax(unit, min, max) => {
                unit.pixely(parent_size).clamp(min.pixely(parent_size), max.pixely(parent_size))
            },
            Self::Add(first, second) => {
                first.pixely(parent_size) + second.pixely(parent_size)
            },
            Self::Sub(first, second) => {
                (first.pixely(parent_size) - second.pixely(parent_size)).max(0.0)
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Align {
    Top(UIUnit),
    Bottom(UIUnit),
    Left(UIUnit),
    Right(UIUnit),
    Center(),
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum UIUnit {
    Zero(),
    Undefined(),
    Pixel(f32),
    Relative(f32),
    RelativeHeight(f32),
    RelativeWidth(f32),
    Rem(f32)
}

impl UIUnit {
    #[inline]
    pub fn pixelx(&self, parent_size: Vec2) -> f32 {
        match self {
            Self::Zero() => {
                0.0
            },
            Self::Undefined() => {
                f32::NAN
            },
            Self::Pixel(pixel) => {
                *pixel
            },
            Self::Relative(percent) | Self::RelativeWidth(percent) =>  {
                parent_size.x * percent
            },
            Self::RelativeHeight(percent) => {
                parent_size.y * percent
            },
            Self::Rem(rem) => {
                *rem
            },
        }
    }

    #[inline]
    pub fn pixely(&self, parent_size: Vec2) -> f32 {
        match self {
            Self::Zero() => {
                0.0
            },
            Self::Undefined() => {
                f32::NAN
            },
            Self::Pixel(pixel) => {
                *pixel
            },
            Self::Relative(percent) | Self::RelativeHeight(percent) =>  {
                parent_size.y * percent
            },
            Self::RelativeWidth(percent) => {
                parent_size.x * percent
            },
            Self::Rem(rem) => {
                *rem
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum BasicAlign {
    Center,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}