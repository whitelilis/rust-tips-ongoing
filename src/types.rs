#[derive(Default)]
pub struct Big {
    pub x: u16,
    pub y: String,
}

pub struct Big2 {
    pub x_b: u8,
    pub y_b: String,
}

#[derive(Default)]
pub struct Bigger {
    pub b1: Big,
    pub b2: Big,
}
