pub struct Settings {
    width: i16,
    height: i16,
}

impl Settings {
    pub fn new(width: i16, height: i16) -> Settings {
        Settings {
            width: width,
            height: height,
        }
    }
}
