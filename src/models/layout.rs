#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct TextRegion {
    pub x: u32,
    pub y: u32,
    pub max_width: u32,
    pub alignment: Alignment,
}

#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Left,
    Center,
    Right,
    CenterLeft,
}

pub const LABEL_SIZE: u32 = 512;

pub struct CommonLayout;
impl CommonLayout {
    pub const BANNER: Rectangle = Rectangle {
        x: 0,
        y: 0,
        width: 512,
        height: 128,
    };

    pub const SCP_NUMBER: TextRegion = TextRegion {
        x: 113,
        y: 165,
        max_width: 240,
        alignment: Alignment::Left,
    };

    pub const OBJECT_CLASS_LABEL: TextRegion = TextRegion {
        x: 25,
        y: 195,
        max_width: 240,
        alignment: Alignment::Left,
    };

    pub const OBJECT_CLASS_TEXT: TextRegion = TextRegion {
        x: 304,
        y: 226,
        max_width: 150,
        alignment: Alignment::Left,
    };
}


pub struct NormalLayout;
impl NormalLayout {
    pub const HAZARD_ICON: Rectangle = Rectangle {
        x: 15,
        y: 256,
        width: 233,
        height: 240,
    };

    pub const USER_IMAGE: Rectangle = Rectangle {
        x: 264,
        y: 256,
        width: 233,
        height: 240,
    };
}

pub struct AlternateLayout;
impl AlternateLayout {
    pub const HAZARD_ICON: Rectangle = Rectangle {
        x: 137,
        y: 256,
        width: 233,
        height: 240,
    };

    pub const SCP_NUMBER: TextRegion = TextRegion {
        x: 268,
        y: 167,
        max_width: 150,
        alignment: Alignment::Left,
    };

    pub const OBJECT_CLASS_TEXT: TextRegion = TextRegion {
        x: 347,
        y: 226,
        max_width: 150,
        alignment: Alignment::Left,
    };
}
