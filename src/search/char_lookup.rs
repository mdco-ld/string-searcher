struct OccurenceVec {
    buffer: Option<Vec<usize>>,
}

impl OccurenceVec {
    fn new() -> Self {
        Self { buffer: None }
    }
    fn add_position(&mut self, position: usize) {
        if let Some(v) = &mut self.buffer {
            v.push(position);
        } else {
            self.buffer = Some(vec![position]);
        }
    }

    fn exists(&self) -> bool {
        self.buffer.is_some()
    }
}

pub struct CharLookup {
    char_lkup: Vec<OccurenceVec>,
}

impl CharLookup {
    pub fn from(text: &[u8]) -> Self {
        let mut lkup = Vec::new();
        lkup.resize_with(256, OccurenceVec::new);
        for (i, ch) in text.iter().enumerate() {
            lkup[*ch as usize].add_position(i);
        }
        Self { char_lkup: lkup }
    }

    pub fn contains(&self, ch: u8) -> bool {
        self.char_lkup[ch as usize].exists()
    }

    pub fn get_positions<'a>(&'a self, ch: u8) -> &'a Vec<usize> {
        const NOTHING: &'static Vec<usize> = &vec![];
        self.char_lkup[ch as usize]
            .buffer
            .as_ref()
            .unwrap_or(NOTHING)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_test_1() {
        let lkup = CharLookup::from("abcabc".as_bytes());
        assert_eq!(lkup.get_positions('a' as u8), &vec![0 as usize, 3]);
        assert_eq!(lkup.get_positions('b' as u8), &vec![1 as usize, 4]);
        assert_eq!(lkup.get_positions('c' as u8), &vec![2 as usize, 5]);
    }
}
