

pub struct ByteReader<T> {
    inner: T,
    pos: usize
}

impl<T> ByteReader<T> {
    pub const fn new(inner: T) -> Self {
        ByteReader { inner, pos: 0 }
    }

    pub const fn new_at(inner: T, position: usize) -> Self {
        ByteReader { inner, pos: position }
    }

    pub fn skip_bytes(&mut self, amount: usize) {
        self.pos += amount;
    }

    pub const fn position(&self) -> usize {
        self.pos
    }

    pub fn set_position(&mut self, position: usize) {
        self.pos = position;
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> ByteReader<T>
where T: AsRef<[u8]> {

    pub fn read_byte(&mut self) -> u8 {
        self.pos += 1;
        self.inner.as_ref()[self.pos - 1]
    }

    pub fn read_bytes(&mut self, len: usize) -> Vec<u8> {
        let start = self.pos;
        self.pos += len;
        self.inner.as_ref()[start..self.pos].to_vec()
    }

    pub fn read_string(&mut self, len: usize) -> String {
        let start = self.pos;
        self.pos += len;
        String::from_utf8_lossy(&self.inner.as_ref()[start..self.pos]).to_string()
    }

    pub fn read_bool(&mut self) -> bool {
        self.pos += 1;
        self.inner.as_ref()[self.pos - 1] != 0
    }

    pub fn read_be_u16(&mut self) -> u16 {
        self.pos += 2;
        u16::from_be_bytes([self.inner.as_ref()[self.pos - 2], self.inner.as_ref()[self.pos - 1]])
    }

    pub fn read_be_u32(&mut self) -> u32 {
        self.pos += 4;
        u32::from_be_bytes([self.inner.as_ref()[self.pos - 4], self.inner.as_ref()[self.pos - 3], self.inner.as_ref()[self.pos - 2], self.inner.as_ref()[self.pos - 1]])
    }

    pub fn read_le_u16(&mut self) -> u16 {
        self.pos += 2;
        u16::from_le_bytes([self.inner.as_ref()[self.pos - 2], self.inner.as_ref()[self.pos - 1]])
    }

    pub fn read_le_u32(&mut self) -> u32 {
        self.pos += 4;
        u32::from_le_bytes([self.inner.as_ref()[self.pos - 4], self.inner.as_ref()[self.pos - 3], self.inner.as_ref()[self.pos - 2], self.inner.as_ref()[self.pos - 1]])
    }
}

impl <T>  ByteReader<T> 
where T: AsRef<[u8]> {}

impl<T: std::fmt::Debug> std::fmt::Debug for ByteReader<T> 
where T: AsRef<[u8]> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ByteReader {{ inner: {:?}, position: {} }}", self.inner.as_ref()[self.pos], self.pos)
    }
}



pub struct ByteWriter {
    buf: Vec<u8>
}

impl ByteWriter {
    
    pub const fn new() -> Self {
        ByteWriter { buf: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        ByteWriter { buf: Vec::with_capacity(capacity) }
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub const fn as_ref(&self) -> &Vec<u8> {
        &self.buf
    }

    pub fn finish(self) -> Vec<u8> {
        self.buf
    }

    pub fn as_mut_ref(&mut self) -> &mut Vec<u8> {
        &mut self.buf
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.buf.push(byte);
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes);
    }

    pub fn write_bool(&mut self, bool: bool) {
        self.buf.push(bool as u8);
    }

    pub fn write_u16(&mut self, number: u16) {
        self.buf.extend(number.to_le_bytes());
    }

    pub fn write_u32(&mut self, number: u32) {
        self.buf.extend(number.to_le_bytes());
    }

    pub fn write_string(&mut self, string: &str) {
        self.buf.extend_from_slice(string.as_bytes());
    }
}