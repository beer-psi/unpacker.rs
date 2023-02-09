//! no_std unpacker for JavaScript code compressed by [packer](http://dean.edwards.name/packer/).
//! 
//! Source code of packer can be found [here](https://github.com/evanw/packer/blob/master/packer.js).
//! 
//! This is a straight port of Tachiyomi's [`:lib-unpacker`](https://github.com/tachiyomiorg/tachiyomi-extensions/blob/master/lib/unpacker/src/main/java/eu/kanade/tachiyomi/lib/unpacker/Unpacker.kt);
#![cfg_attr(not(test), no_std)]
#![feature(test, let_chains)]

#[cfg(test)]
mod tests;

extern crate alloc;
use alloc::{string::{String, ToString}, vec::Vec};

struct SubstringExtractor {
    text: String,
    pub start_index: usize,
}

impl SubstringExtractor {
    pub fn new(text: String) -> Self {
        SubstringExtractor { text, start_index: 0 }
    }

    pub fn substring_before<T: AsRef<str>>(&mut self, text: T) -> String {
        let text = text.as_ref();
        match self.text[self.start_index..].find(text) {
            Some(index) => {
                let index = self.start_index + index;
                let substr = &self.text[self.start_index..index];
                self.start_index = index + text.len();
                String::from(substr)
            }
            None => String::new()
        }
    }

    pub fn substring_between<T: AsRef<str>>(&mut self, left: T, right: T) -> String {
        let left = left.as_ref();
        let right = right.as_ref();

        if let Some(index) = self.text[self.start_index..].find(left) 
           && let Some(right_index) = self.text[self.start_index + index + left.len()..].find(right) {
            let index = self.start_index + index;
            let right_index = right_index + index + left.len();

            let left_index = index + left.len();
            self.start_index = right_index + right.len();
            String::from(&self.text[left_index..right_index])
        } else {
            String::new()
        }
    }
}

pub fn unpack<T: AsRef<str>>(packed: T) -> String {
    let packed_raw = packed.as_ref().to_string();
    let left_index = packed_raw.find("}('").unwrap_or(0) + 3;
    let right_index = packed_raw.find(".split('|'),0,{}))").unwrap_or(0);
    let packed = String::from(&packed_raw[left_index..right_index]).replace("\\'", "\"");

    let mut parser = SubstringExtractor::new(packed);

    let data = parser.substring_before("',");
    if data.is_empty() {
        return String::new()
    }

    let dict_str = parser.substring_between("'", "'");
    let dictionary = dict_str.split('|').collect::<Vec<_>>();
    let len = dictionary.len();

    let mut accum = String::new();
    let mut ret = String::new();
    for char in data.chars() {
        if char.is_ascii_alphanumeric() {
            accum.push(char)
        } else {
            if !accum.is_empty() {
                let index = parse_radix_62(&accum);
            
                if index >= len || dictionary[index].is_empty() {
                    ret.push_str(&accum);
                } else {
                    ret.push_str(dictionary[index]);
                }
                accum = String::new();
            }
            ret.push(char);
        }
    }
    ret
}

fn parse_radix_62<T: AsRef<str>>(data: T) -> usize {
    let data = data.as_ref();
    let mut result = 0;
    for char in data.chars() {
        result = result * 62 + match char {
            ..='9' => char as usize - '0' as usize,
            'a'.. => char as usize - ('a' as usize - 10),
            _ => char as usize - ('A' as usize - 36)
        }
    }
    result
}
