pub trait AsBytes: Copy {
    fn as_bytes(&self) -> &[u8];
}

impl AsBytes for &str {
    fn as_bytes(&self) -> &[u8] {
        return (*self).as_bytes();
    }
}

impl AsBytes for &[u8] {
    fn as_bytes(&self) -> &[u8] {
        return *self;
    }
}

pub enum Endian {
    Big,
    Little,
}
