// 0x05. Rust - Structs and Methods

pub struct StringHolder<'a> {
    pub text: &'a str,
}

impl<'a> StringHolder<'a> {
    pub fn get_string(&self) -> &'a str {
        self.text
    }
}