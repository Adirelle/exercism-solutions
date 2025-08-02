pub fn abbreviate(phrase: &str) -> String {
    Pairs::new(phrase.chars())
        .filter(|(a, b)|
            b.is_alphabetic() &&
            a.map(|a|
                a == '-'
                || a == '_'
                || a.is_whitespace()
                || (a.is_lowercase() && b.is_uppercase())
            ).unwrap_or(true)
        )
        .map(|(_, b)| b.to_ascii_uppercase())
        .collect()
}

struct Pairs<I: Iterator> {
    inner: I,
    prev: Option<I::Item>
}

impl<I: Iterator> Pairs<I> {
    pub fn new(inner: I) -> Self {
        Self { inner, prev: None }
    }
}

impl<I: Iterator> Iterator for Pairs<I>
    where I::Item: Copy
{
    type Item = (Option<I::Item>, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let prev = self.prev.take();
        self.prev = self.inner.next();
        self.prev.map(|p| (prev, p))
    }
}
