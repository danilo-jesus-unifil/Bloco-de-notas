#[derive(Debug, Default)]
pub struct SearchState {
    pub query: String,
    pub replacement: String,
    pub last_match: Option<usize>,
    pub message: Option<String>,
}

impl SearchState {
    pub fn clear_result(&mut self) {
        self.last_match = None;
        self.message = None;
    }

    pub fn find_next(&mut self, text: &str, from_char: usize) -> Option<(usize, usize)> {
        if self.query.is_empty() {
            self.last_match = None;
            self.message = Some("Digite um texto para localizar.".to_owned());
            return None;
        }

        let text_char_count = text.chars().count();
        let start_char = from_char.min(text_char_count);
        let start_byte = byte_index_for_char(text, start_char);
        let found = text[start_byte..]
            .find(&self.query)
            .map(|offset| start_byte + offset)
            .or_else(|| text[..start_byte].find(&self.query));

        self.apply_result(text, found)
    }

    pub fn find_previous(&mut self, text: &str, before_char: usize) -> Option<(usize, usize)> {
        if self.query.is_empty() {
            self.last_match = None;
            self.message = Some("Digite um texto para localizar.".to_owned());
            return None;
        }

        let before_byte = byte_index_for_char(text, before_char.min(text.chars().count()));
        let found = text[..before_byte]
            .rfind(&self.query)
            .or_else(|| text.rfind(&self.query));
        self.apply_result(text, found)
    }

    pub fn replace_first(&mut self, text: &str, from_char: usize) -> Option<String> {
        let (start_char, end_char) = self.find_next(text, from_char)?;
        let start_byte = byte_index_for_char(text, start_char);
        let end_byte = byte_index_for_char(text, end_char);
        let mut replaced = String::with_capacity(text.len() + self.replacement.len());
        replaced.push_str(&text[..start_byte]);
        replaced.push_str(&self.replacement);
        replaced.push_str(&text[end_byte..]);
        Some(replaced)
    }

    pub fn replace_all(&mut self, text: &str) -> Option<String> {
        if self.query.is_empty() {
            self.message = Some("Digite um texto para localizar.".to_owned());
            return None;
        }

        let count = text.matches(&self.query).count();
        if count == 0 {
            self.message = Some("Nenhuma correspondência encontrada.".to_owned());
            return None;
        }

        self.message = Some(format!("{count} ocorrência(s) substituída(s)."));
        Some(text.replace(&self.query, &self.replacement))
    }

    fn apply_result(&mut self, text: &str, found_byte: Option<usize>) -> Option<(usize, usize)> {
        match found_byte {
            Some(start_byte) => {
                let end_byte = start_byte + self.query.len();
                let start_char = text[..start_byte].chars().count();
                let end_char = text[..end_byte].chars().count();
                self.last_match = Some(start_char);
                self.message = Some("Correspondência encontrada.".to_owned());
                Some((start_char, end_char))
            }
            None => {
                self.last_match = None;
                self.message = Some("Nenhuma correspondência encontrada.".to_owned());
                None
            }
        }
    }
}

fn byte_index_for_char(text: &str, char_index: usize) -> usize {
    text.char_indices()
        .nth(char_index)
        .map_or(text.len(), |(byte_index, _)| byte_index)
}

#[cfg(test)]
mod tests {
    use super::SearchState;

    #[test]
    fn wraps_search_to_the_beginning() {
        let mut search = SearchState {
            query: "abc".into(),
            ..Default::default()
        };
        assert_eq!(search.find_next("abc 123 abc", 11), Some((0, 3)));
    }

    #[test]
    fn replaces_all_occurrences_as_one_result() {
        let mut search = SearchState {
            query: "a".into(),
            replacement: "b".into(),
            ..Default::default()
        };
        assert_eq!(search.replace_all("a a"), Some("b b".into()));
    }

    #[test]
    fn returns_character_ranges_for_unicode_search() {
        let mut search = SearchState {
            query: "café".into(),
            replacement: "chá".into(),
            ..Default::default()
        };
        assert_eq!(search.find_next("🙂 café", 0), Some((2, 6)));
        assert_eq!(search.replace_first("🙂 café", 0), Some("🙂 chá".into()));
    }

    #[test]
    fn finds_previous_unicode_match_without_splitting_bytes() {
        let mut search = SearchState {
            query: "é".into(),
            ..Default::default()
        };
        assert_eq!(search.find_previous("café é", 6), Some((5, 6)));
    }
}
