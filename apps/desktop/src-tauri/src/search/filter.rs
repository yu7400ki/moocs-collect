#![allow(dead_code)]

use std::mem;
use tantivy::tokenizer::{Token, TokenFilter, TokenStream, Tokenizer};
use unicode_normalization::UnicodeNormalization;

const DEFAULT_TEMP_STRING_CAPACITY: usize = 100;

#[derive(Debug, Clone)]
pub struct UnicodeNormalizer {
    mode: Kind,
}

impl UnicodeNormalizer {
    pub fn normalize_into(&self, text: &str, out: &mut String) {
        out.clear();
        match self.mode {
            Kind::NFC => out.extend(text.nfc()),
            Kind::NFD => out.extend(text.nfd()),
            Kind::NFKC => out.extend(text.nfkc()),
            Kind::NFKD => out.extend(text.nfkd()),
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Kind {
    /// Normalization Form Canonical Composition.
    NFC,
    /// Normalization Form Canonical Decomposition.
    NFD,
    /// Normalization Form Compatibility Composition.
    NFKC,
    /// Normalization Form Compatibility Decomposition.
    NFKD,
}

impl From<Kind> for UnicodeNormalizer {
    fn from(mode: Kind) -> Self {
        UnicodeNormalizer { mode }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UnicodeNormalizerTokenFilter {
    mode: Kind,
}

impl UnicodeNormalizerTokenFilter {
    pub fn new(mode: Kind) -> Self {
        Self { mode }
    }
}

impl From<Kind> for UnicodeNormalizerTokenFilter {
    fn from(mode: Kind) -> Self {
        UnicodeNormalizerTokenFilter { mode }
    }
}

impl TokenFilter for UnicodeNormalizerTokenFilter {
    type Tokenizer<T: Tokenizer> = UnicodeNormalizerFilterWrapper<T>;

    fn transform<T: Tokenizer>(self, token_stream: T) -> Self::Tokenizer<T> {
        UnicodeNormalizerFilterWrapper::new(token_stream, self.mode)
    }
}

#[derive(Debug)]
pub struct UnicodeNormalizerTokenStream<'a, T> {
    normalizer: UnicodeNormalizer,
    tail: T,
    buffer: &'a mut String,
}

impl<'a, T> UnicodeNormalizerTokenStream<'a, T> {
    pub fn new(tail: T, buffer: &'a mut String, normalizer: UnicodeNormalizer) -> Self {
        Self {
            normalizer,
            tail,
            buffer,
        }
    }
}

impl<T: TokenStream> TokenStream for UnicodeNormalizerTokenStream<'_, T> {
    fn advance(&mut self) -> bool {
        if !self.tail.advance() {
            return false;
        }

        let text = &self.tail.token().text;
        // Fast-path: ASCII remains unchanged under NFC/NFD/NFKC/NFKD
        if text.is_ascii() {
            return true;
        }

        self.normalizer.normalize_into(text, self.buffer);
        mem::swap(&mut self.tail.token_mut().text, self.buffer);
        true
    }

    fn token(&self) -> &Token {
        self.tail.token()
    }

    fn token_mut(&mut self) -> &mut Token {
        self.tail.token_mut()
    }
}

#[derive(Debug, Clone)]
pub struct UnicodeNormalizerFilterWrapper<T> {
    mode: Kind,
    inner: T,
    buffer: String,
}

impl<T> UnicodeNormalizerFilterWrapper<T> {
    pub fn new(inner: T, mode: Kind) -> Self {
        Self {
            mode,
            inner,
            buffer: String::with_capacity(DEFAULT_TEMP_STRING_CAPACITY),
        }
    }
}

impl<T: Tokenizer> Tokenizer for UnicodeNormalizerFilterWrapper<T> {
    type TokenStream<'a> = UnicodeNormalizerTokenStream<'a, T::TokenStream<'a>>;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> Self::TokenStream<'a> {
        self.buffer.clear();
        UnicodeNormalizerTokenStream::new(
            self.inner.token_stream(text),
            &mut self.buffer,
            self.mode.into(),
        )
    }
}
