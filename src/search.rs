mod char_lookup;

use self::char_lookup::CharLookup;

#[derive(PartialEq, Debug)]
pub struct Match {
    position: usize,
}

pub struct SearchBuilder {
    pattern: String,
    case_sensitive: bool,
}

fn get_lowercase_string_if(condition: bool, text: &str) -> Option<String> {
    if condition {
        Some(text.to_lowercase())
    } else {
        None
    }
}

impl SearchBuilder {
    pub fn from_pattern(pattern: &str) -> Self {
        Self {
            pattern: pattern.to_owned(),
            case_sensitive: true,
        }
    }

    pub fn case_sensitive(self, case_sensitive: bool) -> Self {
        Self {
            pattern: self.pattern,
            case_sensitive,
        }
    }

    pub fn build(self) -> Searcher {
        Searcher::create(self.pattern, self.case_sensitive)
    }
}

pub struct Searcher {
    pattern: Vec<u8>,
    char_lkup: char_lookup::CharLookup,
    case_sensitive: bool,
}

impl Searcher {
    fn create(pattern: String, case_sensitive: bool) -> Self {
        let lower = get_lowercase_string_if(!case_sensitive, &pattern);
        let pattern = if let Some(lower) = &lower {
            lower.as_bytes()
        } else {
            pattern.as_bytes()
        };
        Self {
            pattern: pattern.to_owned(),
            char_lkup: CharLookup::from(pattern),
            case_sensitive,
        }
    }
    pub fn naive_search(&self, content: &str) -> Vec<Match> {
        let mut result = vec![];
        if self.pattern.len() == 0 {
            return result;
        }
        let mut position = 0;
        let lowercase_content = get_lowercase_string_if(!self.case_sensitive, content);
        let content = if let Some(lower) = &lowercase_content {
            lower.as_bytes()
        } else {
            content.as_bytes()
        };
        while position + self.pattern.len() <= content.len() {
            if content[position..(position + self.pattern.len())] == self.pattern {
                result.push(Match { position });
            }
            position += 1;
        }
        result
    }
    pub fn search(&self, content: &str) -> Vec<Match> {
        let mut result = vec![];
        if self.pattern.len() == 0 {
            return result;
        }
        let mut position = self.pattern.len() - 1;
        let lowercase_content = get_lowercase_string_if(!self.case_sensitive, content);
        let content = if let Some(lower) = &lowercase_content {
            lower.as_bytes()
        } else {
            content.as_bytes()
        };
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
        let mut result: Option<Vec<Match>> = None;
        for i in self.char_lkup.get_positions(content[position]).iter().rev() {
            if position - i + self.pattern.len() <= content.len()
                && self.is_match_at_pos(content, position - i)
            {
                if let Some(v) = &mut result {
                    v.push(Match {
                        position: position - i,
                    });
                } else {
                    result = Some(vec![Match {
                        position: position - i,
                    }]);
                }
            }
        }
        result
    }

    fn is_match_at_pos(&self, text: &[u8], position: usize) -> bool {
        text[position..(position + self.pattern.len())] == self.pattern
    }

    fn contains(&self, c: u8) -> bool {
        self.char_lkup.contains(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_search() {
        let searcher = SearchBuilder::from_pattern("bc").build();
        let mats = searcher.naive_search("abcd".into());
        assert_eq!(mats.len(), 1);
        let mat1 = mats.first().unwrap();
        assert_eq!(mat1.position, 1);
    }

    #[test]
    fn no_matches() {
        let searcher = SearchBuilder::from_pattern("bd").build();
        let mats = searcher.naive_search("abcd".into());
        assert_eq!(mats.len(), 0);
    }

    #[test]
    fn empty_content() {
        let searcher = SearchBuilder::from_pattern("abc").build();
        let mats = searcher.naive_search("".into());
        assert_eq!(mats.len(), 0);
    }

    #[test]
    fn search_string_too_long() {
        let searcher = SearchBuilder::from_pattern("abcdefgh").build();
        let mats = searcher.naive_search("abc");
        assert_eq!(mats.len(), 0);
    }

    #[test]
    fn two_matches() {
        let searcher = SearchBuilder::from_pattern("abc").build();
        let mats = searcher.naive_search("xabc123abcx".into());
        assert_eq!(mats.len(), 2);
        assert_eq!(mats, vec![Match { position: 1 }, Match { position: 7 }]);
    }

    #[test]
    fn case_no_match() {
        let searcher = SearchBuilder::from_pattern("bcd").build();
        let mats = searcher.search("ABCDEF");
        assert_eq!(mats.len(), 0);
    }

    #[test]
    fn longer_test() {
        let content = "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.";
        let searcher = SearchBuilder::from_pattern("dolor").build();
        let mats = searcher.naive_search(content);
        let mats2 = searcher.search(content);
        assert_eq!(mats2, mats);
    }

    #[test]
    fn case_insensitive_basic() {
        let searcher = SearchBuilder::from_pattern("AbCd")
            .case_sensitive(false)
            .build();
        let mats = searcher.search("123aBcd123");
        assert_eq!(mats, vec![Match { position: 3 }]);
    }

    #[test]
    fn random_test_1() {
        let searcher = SearchBuilder::from_pattern("abcabc").build();
        let mats = searcher.search("abcabcabc");
        assert_eq!(mats, vec![Match { position: 0 }, Match { position: 3 }]);
    }
}
