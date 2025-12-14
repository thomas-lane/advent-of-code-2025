use std::{fs, mem};

pub fn input_to_string(input_name: &str) -> String {
    let file_path = format!("inputs/{}", input_name);
    let contents = fs::read_to_string(file_path).unwrap();

    contents
}

#[derive(Clone)]
pub struct BitArray {
    len: usize,
    data: Box<[u64]>
}

impl BitArray {
    const BITS_PER_WORD: usize = mem::size_of::<u64>() * 8;

    pub fn new(len: usize) -> Self {
        if len == 0 {
            return Self { len, data: vec![0u64; 0].into_boxed_slice() };
        }

        let words = (len + Self::BITS_PER_WORD - 1) / Self::BITS_PER_WORD;

        Self { len, data: vec![0u64; words].into_boxed_slice() }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, index: usize) -> bool {
        let (word_index, bit_index) = Self::get_packed_indices(index);
        self.data[word_index] & (1u64 << bit_index) != 0
    }

    pub fn set(&mut self, index: usize, value: bool) {
        let (word_index, bit_index) = Self::get_packed_indices(index);
        let mask = 1u64 << bit_index;
        
        if value {
            self.data[word_index] |= mask;
        } else {
            self.data[word_index] &= !mask;
        }
    }

    pub fn xor(&self, other: &BitArray) -> BitArray {
        assert_eq!(self.len, other.len);

        let mut result = BitArray::new(self.len);

        for i in 0..self.data.len() {
            result.data[i] = self.data[i] ^ other.data[i];
        }

        result
    }

    fn get_packed_indices(index: usize) -> (usize, usize) {
        ( index / Self::BITS_PER_WORD, index % Self::BITS_PER_WORD )
    }
}

impl std::fmt::Display for BitArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut to_print = String::with_capacity(self.len());
        for i in 0..self.len() {
            to_print.push(if self.get(i) { '1' } else { '0' });
        }
        write!(f, "{}", to_print)
    }
}

impl PartialEq<BitArray> for BitArray {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }

        for i in 0..self.len {
            if self.get(i) != other.get(i) {
                return false;
            }
        }

        true
    }
}
