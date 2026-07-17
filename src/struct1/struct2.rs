pub struct Book <'a> {
    pub title: &'a str,
    pub quote: &'a str,
}
impl<'a> Book<'a> {
    pub fn longer_field(&self) -> &str{
        if self.title.len() > self.quote.len() {
            self.title
        }else {
            self.quote
        }
    }
}

pub fn most_interesting_word<'a>(text: &'a str, boring_word: &'a str) -> &'a str {
    let first_word = text.split_whitespace().next().unwrap_or(" ");
    if first_word == boring_word {
        let second_word = text.split_whitespace().nth(1).unwrap_or(first_word);
        second_word
    }else {
        first_word
    }
}