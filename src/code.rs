use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref MORSE: HashMap<char, &'static str> = {
        let codes = vec![
            ('a', ".-"),
            ('b', "-..."),
            ('c', "-.-."),
            ('d', "-.."),
            ('e', "."),
            ('f', "..-."),
            ('g', "--."),
            ('h', "...."),
            ('i', ".."),
            ('j', ".---"),
            ('k', "-.-"),
            ('l', ".-.."),
            ('m', "--"),
            ('n', "-."),
            ('o', "---"),
            ('p', ".--."),
            ('q', "--.-"),
            ('r', ".-."),
            ('s', "..."),
            ('t', "-"),
            ('u', "..-"),
            ('v', "...-"),
            ('w', ".--"),
            ('x', "-..-"),
            ('y', "-.--"),
            ('z', "--.."),
        ];

        let mut morse = HashMap::new();

        for (letter, code) in codes {
            morse.insert(letter, code);
        }

        morse
    };
}
