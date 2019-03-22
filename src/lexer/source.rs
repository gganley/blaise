use itertools::PeekingNext;
use std::iter::{Iterator, Peekable};
use std::str::Chars;

pub struct Source<'a> {
    source_string: Peekable<Chars<'a>>,
    current: Option<char>,
}

impl<'a> Source<'a> {
    pub fn new(input: &'a str) -> Self {
        Source {
            source_string: input.chars().peekable(),
            current: None,
        }
    }

    pub fn current(&self) -> Option<char> {
        self.current
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.source_string.peek()
    }
}

impl<'a> Iterator for Source<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.current = self.source_string.next();
        self.current
    }
}

impl<'a> PeekingNext for Source<'a> {
    fn peeking_next<F>(&mut self, good: F) -> Option<Self::Item>
    where
        F: FnOnce(&Self::Item) -> bool,
    {
        let pass = match self.peek() {
            Some(c) if good(c) => true,
            _ => false,
        };

        return if pass { self.next() } else { None };
    }
}
