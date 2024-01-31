#![warn(rust_2018_idioms)]
 
 
 pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
 }

 impl StrSplit<'_> {
    pub fn new(heystack: &str, delimiter: &str) -> Self {
        Self { 
            remainder: heystack, 
            delimiter, 
        }
    }
 }

 impl Interator for StrSplit<'_> {
    type Item = &str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)

        } else if  self.remainder.is_empty() {
            None
        } else {
            let rest = self.delimiter;
            self.remainder = &[];
            Some(rest)
        }
    }
 }

 #[test]
 fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(heystack, " ");
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter());
 }