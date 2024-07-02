#[derive(Debug, Clone)]
pub struct Bytes {
  pub bytes: Vec<u8>,
  pub position: usize
}

impl Bytes {
  pub fn from(context: String) -> Bytes {
    let mut bytes: Vec<u8> = context.bytes().collect();
    bytes.push(b'\x00');
    Bytes { bytes, position: 0 }
  }

  pub fn null() -> Bytes {
    Bytes::from(String::new())
  }

  pub fn getc(&mut self) -> Option<char> {
    if self.bytes.len() - 1 == self.position { return None }

    if (self.bytes.len() - 1) > self.position {
      self.position += 1;
    }

    let byte: u8 = self.bytes[self.position - 1];
    if byte == b'\x00' { return None }
    Some(char::from(byte))
  }

  pub fn sum(&mut self) {
    if self.bytes.len() - 1 == self.position { return; }

    if (self.bytes.len() - 1) > self.position {
      self.position += 1;
    }
  }

  pub fn after(&mut self) -> Option<char> {
    if self.bytes.len() - 1 == self.position { return None }

    let byte: u8 = self.bytes[self.position];
    if byte == b'\x00' { return None }
    Some(char::from(byte))
  }

  pub fn ungetc(&mut self) {
    self.position -= 1; 
  }
}