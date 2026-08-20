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

    pub fn find_next(&mut self, text: &str, from: usize) -> Option<(usize, usize)> {
        if self.query.is_empty() {
            self.last_match = None;
            self.message = Some("Digite um texto para localizar.".to_owned());
            return None;
        }

        let start = from.min(text.len());
        let found = text[start..]
            .find(&self.query)
            .map(|offset| (start + offset, start + offset + self.query.len()))
            .or_else(|| {
                text[..start]
                    .find(&self.query)
                    .map(|offset| (offset, offset + self.query.len()))
            });

        match found {
            Some(range) => {
                self.last_match = Some(range.0);
                self.message = Some("Correspondência encontrada.".to_owned());
                Some(range)
            }
            None => {
                self.last_match = None;
                self.message = Some("Nenhuma correspondência encontrada.".to_owned());
                None
            }
        }
    }

    pub fn replace_first(&mut self, text: &str, from: usize) -> Option<String> {
        let (start, end) = self.find_next(text, from)?;
        let mut replaced = String::with_capacity(text.len() + self.replacement.len());
        replaced.push_str(&text[..start]);
        replaced.push_str(&self.replacement);
        replaced.push_str(&text[end..]);
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
}
