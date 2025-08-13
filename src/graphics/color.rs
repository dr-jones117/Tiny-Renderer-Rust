#[allow(dead_code)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[allow(dead_code)]
pub const RED: RGBA = RGBA {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

#[allow(dead_code)]
pub const GREEN: RGBA = RGBA {
    r: 0,
    g: 255,
    b: 0,
    a: 255,
};

#[allow(dead_code)]
pub const BLUE: RGBA = RGBA {
    r: 0,
    g: 0,
    b: 255,
    a: 255,
};

#[allow(dead_code)]
pub const PURPEL: RGBA = RGBA {
    r: 174,
    g: 50,
    b: 220,
    a: 255,
};
