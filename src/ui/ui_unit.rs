use crate::primitives::Vec2;



#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum UiSize {
    Auto(),
    AutoMax(UIUnit),
    Size(UIUnit),
    Add(UIUnit, UIUnit),
    Sub(UIUnit, UIUnit),
    Fill(),
    SetMin(UIUnit, UIUnit),
    SetMax(UIUnit, UIUnit),
}

impl UiSize {
    #[inline]
    pub fn width(&self, parent_size: Vec2) -> f32 {
        match self {
            Self::Auto() => {
                100.0
            },
            Self::AutoMax(max) => {
                max.pixelx(parent_size).min(100.0)
            },
            Self::Size(unit) => {
                unit.pixelx(parent_size)
            },
            Self::Add(first, second) => {
                first.pixelx(parent_size) + second.pixelx(parent_size)
            },
            Self::Sub(first, second) => {
                (first.pixelx(parent_size) - second.pixelx(parent_size)).max(0.0)
            },
            UiSize::Fill() => {
                parent_size.x
            },
            UiSize::SetMin(unit, min) => {
                unit.pixelx(parent_size).max(min.pixelx(parent_size))
            },
            UiSize::SetMax(unit, max) => {
                unit.pixelx(parent_size).min(max.pixelx(parent_size))
            }, 
        }
    }

    #[inline]
    pub fn height(&self, parent_size: Vec2) -> f32 {
        match self {
            Self::Auto() => {
                60.0
            },
            Self::AutoMax(max) => {
                max.pixely(parent_size).min(100.0)
            },
            Self::Size(unit) => {
                unit.pixely(parent_size)
            },
            Self::Add(first, second) => {
                first.pixely(parent_size) + second.pixely(parent_size)
            },
            Self::Sub(first, second) => {
                (first.pixely(parent_size) - second.pixely(parent_size)).max(0.0)
            },
            UiSize::Fill() => {
                parent_size.y
            },
            UiSize::SetMin(unit, min) => {
                unit.pixely(parent_size).max(min.pixely(parent_size))
            },
            UiSize::SetMax(unit, max) => {
                unit.pixely(parent_size).min(max.pixely(parent_size))
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
            Self::Zero() => {
                0.0
            }
        }
    }

    #[inline]
    pub fn pixely(&self, parent_size: Vec2) -> f32 {
        match self {
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
            Self::Zero() => {
                0.0
            }
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