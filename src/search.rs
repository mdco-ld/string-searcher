use std::cmp::min;

#[derive(PartialEq, Debug)]
pub struct Match {
    position: usize,
}

pub struct Searcher {
    pattern: Vec<u8>,
    char_lkup: Box<[bool; 256]>,
    case_sensitive: bool,
}

impl Searcher {
    pub fn create(pattern: String) -> Self {
        let pattern = pattern.as_bytes().to_owned();
        let mut char_lkup = Box::new([false; 256]);
        for ch in pattern.iter() {
            char_lkup[*ch as usize] = true;
        }
        Self {
            pattern,
            char_lkup,
            case_sensitive: true,
        }
    }
    pub fn case_sensitive(self, case_sensitive: bool) -> Self {
        Self {
            pattern: self.pattern,
            char_lkup: self.char_lkup,
            case_sensitive,
        }
    }
    pub fn search(&self, content: String) -> Vec<Match> {
        let mut result = vec![];
        if self.pattern.len() == 0 {
            return result;
        }
        let mut position = self.pattern.len() - 1;
        let content = content.as_bytes();
        while position < content.len() {
            if self.contains(content[position]) {
                let position_match = self.search_position(content, position);
                if let Some(mut mat) = position_match {
                    result.append(&mut mat);
                }
            }
            position += self.pattern.len();
        }
        result
    }
    fn search_position(&self, content: &[u8], position: usize) -> Option<Vec<Match>> {
        let start = position + 1 - self.pattern.len();
        let mut result = None;
        for i in start..min(position + 1, content.len() - self.pattern.len()) {
            if content[i..(i + self.pattern.len())] == self.pattern {
                if !result.is_some() {
                    result = Some(vec![]);
                }
                result = result.map(|mut v| {
                    v.push(Match { position: i });
                    v
                });
            }
        }
        result
    }
    fn contains(&self, c: u8) -> bool {
        self.char_lkup[c as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_search() {
        let searcher = Searcher::create("bc".into());
        let mats = searcher.search("abcd".into());
        assert_eq!(mats.len(), 1);
        let mat1 = mats.first().unwrap();
        assert_eq!(mat1.position, 1);
    }

    #[test]
    fn no_matches() {
        let searcher = Searcher::create("bd".into());
        let mats = searcher.search("abcd".into());
        assert_eq!(mats.len(), 0);
    }

    #[test]
    fn empty_content() {
        let searcher = Searcher::create("abc".into());
        let mats = searcher.search("".into());
        assert_eq!(mats.len(), 0);
    }

    #[test]
    fn search_string_too_long() {
        let searcher = Searcher::create("abcdefgh".into());
        let mats = searcher.search("abc".into());
        assert_eq!(mats.len(), 0);
    }

    #[test]
    fn two_matches() {
        let searcher = Searcher::create("abc".into());
        let mats = searcher.search("xabc123abcx".into());
        assert_eq!(mats.len(), 2);
    }

    #[test]
    fn longer_test() {
        let content = "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.";
        let searcher = Searcher::create("dolor".into());
        let mats = searcher.search(content.into());
        let content = content.as_bytes();
        for mat in mats {
            assert_eq!(
                "dolor".as_bytes(),
                &content[mat.position..(mat.position + 5)]
            );
        }
    }
}
