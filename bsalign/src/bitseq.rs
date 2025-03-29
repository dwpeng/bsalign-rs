#[derive(Debug, Default)]
pub struct BitSeq {
    size: usize,
    buffer: Vec<u8>,
    packed: bool,
}

#[inline]
fn base_to_2bit(base: u8) -> u8 {
    match base {
        b'A' | b'a' => 0,
        b'C' | b'c' => 1,
        b'G' | b'g' => 2,
        b'T' | b't' => 3,
        _ => 4,
    }
}

#[inline]
fn base_from_2bit(base: u8) -> u8 {
    match base {
        0 => b'A',
        1 => b'C',
        2 => b'G',
        3 => b'T',
        _ => b'N',
    }
}

impl BitSeq {
    pub fn with_capacity(size: usize) -> BitSeq {
        BitSeq {
            size,
            buffer: Vec::with_capacity(size / 4),
            packed: true,
        }
    }

    // Keep these constructors for backward compatibility
    pub fn from_sequence(seq: &str, packed: bool) -> Self {
        let mut bitseq = BitSeq::default();
        bitseq.set_sequence(seq, packed);
        bitseq
    }

    pub fn from_bits(bits: &[u8], packed: bool) -> Self {
        let mut bitseq = BitSeq::default();
        bitseq.set_bits(bits, packed);
        bitseq
    }
}

impl BitSeq {
    pub fn set_sequence(&mut self, seq: &str, packed: bool) -> &mut Self {
        let size = seq.len();
        let bytes = seq.as_bytes();

        self.size = size;
        self.packed = packed;
        self.buffer.clear();

        if packed {
            self.buffer.resize((size + 3) / 4, 0);
            for i in 0..size {
                let base = base_to_2bit(bytes[i]);
                self.buffer[i / 4] |= base << (2 * (i % 4));
            }
        } else {
            self.buffer.reserve(size);
            for i in 0..size {
                let base = base_to_2bit(bytes[i]);
                self.buffer.push(base);
            }
        }
        self
    }

    pub fn set_bits(&mut self, bits: &[u8], packed: bool) -> &mut Self {
        let size = bits.len();
        self.size = size;
        self.packed = packed;
        self.buffer.clear();

        if packed {
            self.buffer.resize((size + 3) / 4, 0);
            for i in 0..size {
                let base = bits[i];
                self.buffer[i / 4] |= base << (2 * (i % 4));
            }
        } else {
            self.buffer.reserve(size);
            for i in 0..size {
                self.buffer.push(bits[i]);
            }
        }
        self
    }

    pub fn to_sequence(&self, buf: &mut String) {
        buf.clear();
        buf.reserve(self.size);
        if self.packed {
            for i in 0..self.size {
                let base = base_from_2bit((self.buffer[i / 4] >> (2 * (i % 4))) & 3);
                buf.push(base as char);
            }
        } else {
            for i in 0..self.size {
                let base = base_from_2bit(self.buffer[i]);
                buf.push(base as char);
            }
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn index_base(&self, index: usize) -> Option<u8> {
        let bit = self.index_bit(index);
        if let Some(b) = bit {
            Some(base_from_2bit(b))
        } else {
            None
        }
    }

    pub fn index_bit(&self, index: usize) -> Option<u8> {
        if index >= self.len() {
            return None;
        }
        let nblock = index / 4;
        let offset = index % 4;
        let bit = self.buffer[nblock] >> (2 * offset) & 0x3;
        Some(bit)
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.buffer.clear();
    }

    pub fn to_string(self) -> String {
        self.into()
    }
}

impl From<&[u8]> for BitSeq {
    fn from(seq: &[u8]) -> Self {
        let mut bitseq = BitSeq::default();
        bitseq.set_bits(seq, false);
        bitseq
    }
}

impl From<&str> for BitSeq {
    fn from(seq: &str) -> Self {
        let mut bitseq = BitSeq::default();
        bitseq.set_sequence(seq, false);
        bitseq
    }
}

impl From<Vec<u8>> for BitSeq {
    fn from(seq: Vec<u8>) -> Self {
        let mut bitseq = BitSeq::default();
        bitseq.set_bits(&seq, false);
        bitseq
    }
}

impl Into<String> for BitSeq {
    fn into(self) -> String {
        let mut buf = String::new();
        self.to_sequence(&mut buf);
        buf
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bitseq() {
        let s = String::from("ATCTGAGT");
        let bitseq = BitSeq::from_sequence(&s, true);
        assert_eq!(bitseq.len(), s.len());

        let mut buf = String::new();
        bitseq.to_sequence(&mut buf);
        assert_eq!(s, buf);

        let bitseq = BitSeq::from_sequence(&s, false);
        assert_eq!(bitseq.len(), s.len());

        let mut buf = String::new();
        bitseq.to_sequence(&mut buf);
        assert_eq!(s, buf);

        let bits = vec![0, 0, 0, 0, 0, 1];
        let bitseq = BitSeq::from_bits(&bits, false);
        assert_eq!(bits.len(), bitseq.len());
        bitseq.to_sequence(&mut buf);
        assert_eq!(buf, "AAAAAC");

        let bitseq = BitSeq::from_bits(&bits, true);
        assert_eq!(bits.len(), bitseq.len());
        bitseq.to_sequence(&mut buf);
        assert_eq!(buf, "AAAAAC");
    }

    #[test]
    fn test_index() {
        let s = String::from("ATCTGAGT");
        let bitseq = BitSeq::from_sequence(&s, true);
        assert_eq!(bitseq.len(), s.len());

        assert_eq!(bitseq.index_base(0), Some(b'A'));
        assert_eq!(bitseq.index_base(1), Some(b'T'));

        assert_eq!(bitseq.index_bit(0), Some(0));
        assert_eq!(bitseq.index_bit(1), Some(3));
    }
}
