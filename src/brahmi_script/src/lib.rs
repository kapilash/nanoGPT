use pyo3::prelude::*;
use std::fs::File;
use std::io::{self};
use std::io::prelude::*;
use std::path::Path;
use std::hash::Hash;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use pyo3::exceptions::PyValueError;
use std::fmt;

#[derive(Debug,Clone,Copy,PartialEq,Eq, Hash, Serialize, Deserialize)]
enum Syllable {
    /// Monophthong
    Mono(u32),

    /// Diphthong (consonant + vowel symbol) or (vowel + vowel_suffix)
    Di(u32, u32),

    /// CVV (consonant + vowel + vowel)
    Cvv(u32, u32, u32),
    
    /// Triphthong
    Cvc(u32, u32, u32),

    /// Consonant + Consonant + Double-Vowel,
    Cvvc(u32, u32, u32, u32),

    /// Double consonant
    Cc(u32, u32),

    /// Meta character
    Meta(u32),
}

impl Syllable {

    fn append_char(&self, str:&mut String, config:&Config) {
        match *self {
            Syllable::Mono(c) => {
                str.push(std::char::from_u32(c).unwrap());
            },
            Syllable::Di(c1, c2) => {
                str.push(std::char::from_u32(c1).unwrap());
                str.push(std::char::from_u32(c2).unwrap());
            },
            Syllable::Cc(c1, c2) => {
                str.push(std::char::from_u32(c1).unwrap());
                str.push(std::char::from_u32(config.virama).unwrap());
                str.push(std::char::from_u32(c2).unwrap());
            },
            Syllable::Cvc(c1, c2, c3) => {
                str.push(std::char::from_u32(c1).unwrap());
                str.push(std::char::from_u32(config.virama).unwrap());
                str.push(std::char::from_u32(c2).unwrap());
                str.push(std::char::from_u32(c3).unwrap());
            },
            Syllable::Cvv(c1, c2, c3) => {
                str.push(std::char::from_u32(c1).unwrap());
                str.push(std::char::from_u32(c2).unwrap());
                str.push(std::char::from_u32(c3).unwrap());
            },
            Syllable::Cvvc(c1, c2, c3, c4) => {
                str.push(std::char::from_u32(c1).unwrap());
                str.push(std::char::from_u32(config.virama).unwrap());
                str.push(std::char::from_u32(c2).unwrap());
                str.push(std::char::from_u32(c3).unwrap());
                str.push(std::char::from_u32(c4).unwrap());
            },
            Syllable::Meta(c) => {
                str.push(std::char::from_u32(c).unwrap());
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SyllableMapping {
    syllable_token : HashMap<Syllable, u32>,
    token_syllable : Vec<Syllable>,
}

impl SyllableMapping {

    // Adds a syllable to the syllabary and returns the encoded token.
    fn syllable_code(&self, syllable : Syllable, config:&Config) -> u32 {
        if let Some(&token) = self.syllable_token.get(&syllable) {
            return token;
        }
        config.unknown
    }

    fn get_syllable(&self, token : u32) -> Option<Syllable> {
        if token < self.token_syllable.len() as u32 {
            Some(self.token_syllable[token as usize])
        } else {
            None
        }
    }

    // Adds a syllable to the syllabary if needed
    fn add_syllable(&mut self, syllable : Syllable) {
        if self.syllable_token.contains_key(&syllable) {
            return ;
        }
        let token = self.token_syllable.len() as u32;
        self.syllable_token.insert(syllable, token);
        self.token_syllable.push(syllable);
    }
}

/// Intermediate representation of various Brahmic Script symbols.
/// The symbols are classified into various categories.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SymbolInfo {
    VowelSuffix(u32),
    VowelSign(u32),
    Vowel(u32),
    Consonant(u32),
    Virama(u32),
    Ignored(u32),
    Digit(u32),
    EndMarker(u32),
    OutOfRange(u32),
}

impl fmt::Display for SymbolInfo {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SymbolInfo::VowelSuffix(c) => write!(f, "VowelSuffix(U{:x}, {})", c, std::char::from_u32(c).unwrap()),
            SymbolInfo::VowelSign(c) => write!(f, "VowelSign(U{:x},{})", c, std::char::from_u32(c).unwrap()),
            SymbolInfo::Vowel(c) => write!(f, "Vowel(U{:x},{})", c, std::char::from_u32(c).unwrap()),
            SymbolInfo::Consonant(c) => write!(f, "Consonant(U{:x},{})", c, std::char::from_u32(c).unwrap()),
            SymbolInfo::Virama(c) => write!(f, "Virama(U{:x},{})", c, std::char::from_u32(c).unwrap()),
            SymbolInfo::Ignored(c) => write!(f, "Ignored(U{:x},{})", c, std::char::from_u32(c).unwrap()),
            SymbolInfo::Digit(c) => write!(f, "Digit(U{:x},{})", c, std::char::from_u32(c).unwrap()),
            SymbolInfo::EndMarker(c) => write!(f, "EndMarker(U{:x}, {})", c, std::char::from_u32(c).unwrap()),
            SymbolInfo::OutOfRange(c) => write!(f, "OutOfRange(U{:x}, {})", c, std::char::from_u32(c).unwrap()),
        }
    }
}


#[derive(Serialize, Deserialize)]
struct Config {
    independent_vowels : Vec<u32>,
    consonants : Vec<u32>,
    vowel_suffixes : Vec<u32>,
    vowel_signs : Vec<u32>,
    reserved : Vec<u32>,
    ignored : Vec<u32>,
    digits : Vec<u32>,
    virama : u32,
    end_of_text: Vec<u32>,
    unknown : u32
}

impl Config {

    fn new_telugu() -> Self {
        let independent_vowels = vec![0xC05, 0xC06, 0xC07, 0xC08, 0xC09, 0xC0A, 0xC0B, 0xC0C, 0xC0E, 0xC0F, 0xC10, 0xC12, 0xC13, 0xC14, 0xC60, 0xC61];
        let consonants = vec![0xC15, 0xC16, 0xC17, 0xC18, 0xC19, 0xC1A, 0xC1B, 0xC1C, 0xC1D, 0xC1E, 0xC1F, 
                              0xC20, 0xC21, 0xC22, 0xC23, 0xC24, 0xC25, 0xC26, 0xC27, 0xC28, 0xC2A, 0xC2B, 0xC2C, 0xC2D, 0xC2E, 0xC2F,
                              0xC30, 0xC31, 0xC32, 0xC33, 0xC34, 0xC35, 0xC36, 0xC37, 0xC38, 0xC39,
                              0xC58, 0xC59, 0xC5A];
        let vowel_suffixes = vec![0xC00, 0xC01, 0xC02, 0xC03, 0xC04];
        let vowel_signs = vec![0xC3E, 0xC3F, 0xC40, 0xC41, 0xC42, 0xC43, 0xC44, 0xC46, 0xC47, 0xC48, 0xC4A, 0xC4B, 0xC4C, 0xC62, 0xC63];
        let virama = 0xC4D;
        let reserved = vec![0xC0D, 0xC11, 0xC29, 0xC3A, 0xC3B,
                            0xC45, 0xC49, 0xC4E, 0xC4F,
                            0xC50, 0xC51, 0xC52, 0xC53, 0xC54, 0xC57,  0xC5B, 0xC5C, 0xC5E, 0xC5F,
                            0xC64, 0xC65, 0xC70, 0xC71, 0xC72, 0xC73, 0xC74, 0xC75, 0xC76];
        let ignored = vec![0xC55, 0x5A, 0xC5D, 0xC77, 0xC78, 0xC79, 0xC7A, 0xC7B, 0xC7C, 0xC7D, 0xC7E, 0xC7F];
        let digits = vec![0xC66, 0xC67, 0xC68, 0xC69, 0xC6A, 0xC6B, 0xC6C, 0xC6D, 0xC6E, 0xC6F];
        let end_of_text = vec![0xC77];
        let unknown = 0xC7F;

        Config {
            independent_vowels,
            consonants,
            vowel_suffixes,
            vowel_signs,
            reserved,
            ignored,
            digits,
            virama,
            end_of_text,
            unknown,
        }
    }

    fn new_devnagari() -> Self {
        let independent_vowels = vec![0x904, 0x905, 0x906, 0x907, 0x908, 0x909, 0x90A, 0x90B, 0x90C, 0x90D, 0x90E, 0x90F, 0x910, 0x911, 0x912, 0x913, 0x914, 0x950, 0x960,0x961];
        let consonants = vec![0x915, 0x916, 0x917, 0x918, 0x919, 0x91A, 0x91B, 0x91C, 0x91D, 0x91E, 0x91F, 
                              0x920, 0x921, 0x922, 0x923, 0x924, 0x925, 0x926, 0x927, 0x928, 0x929, 0x92A, 0x92B, 0x92C, 0x92D, 0x92E, 0x92F,
                              0x930, 0x931, 0x932, 0x933, 0x934, 0x935, 0x936, 0x937, 0x938, 0x939,
                              0x958, 0x959, 0x95A,0x95B, 0x95C,0x95D,0x95E,0x95F];
        let vowel_suffixes = vec![0x900, 0x901, 0x902, 0x903];
        let vowel_signs = vec![0x93A, 0x93B, 0x93C, 0x93D, 0x93E, 0x93F, 0x940, 0x941, 0x942,0x943, 0x944, 0x945, 0x946, 0x947, 0x948, 0x949, 0x94A, 0x94B, 0x94C, 0x94D, 0x94E, 0x94F];
        let virama = 0x94D;
        let reserved = vec![0x970,0x971,0x972,0x973,0x974,0x975,0x976,0x977,0x978,0x979,0x97A,0x97B,0x97C,0x97D,0x97E,0x97F];
        let ignored = vec![];
        let digits = vec![0x966, 0x967, 0x968, 0x969, 0x96A, 0x96B, 0x96C, 0x96D, 0x96E, 0x96F];
        let end_of_text = vec![0x964, 0x965];
        let unknown = 0x97F;

        Config {
            independent_vowels,
            consonants,
            vowel_suffixes,
            vowel_signs,
            reserved,
            ignored,
            digits,
            virama,
            end_of_text,
            unknown,
        }
    }

    /// Converts a unicode character to a SymbolInfo instance.
    pub fn to_symbol_info(&self, c:u32) -> SymbolInfo {
        if self.is_vowel_suffix(c) {
            return SymbolInfo::VowelSuffix(c);
        }
        if self.is_vowel_symbol(c) {
            return SymbolInfo::VowelSign(c);
        }
        if self.is_vowel(c) {
            return SymbolInfo::Vowel(c);
        }
        if self.is_consonant(c) {
            return SymbolInfo::Consonant(c);
        }
        if self.is_virama(c) {
            return SymbolInfo::Virama(c);
        }
        if self.is_ignored(c) {
            for res in self.reserved.iter() {
                if c == *res {
                    return SymbolInfo::Ignored(c);
                }
            }
            for ig in self.ignored.iter() {
                if c == *ig {
                    return SymbolInfo::Ignored(c);
                }
            }
        }
        if self.is_digit(c) {
            return SymbolInfo::Digit(c);
        }
        for eot in self.end_of_text.iter() {
            if c == *eot {
                return SymbolInfo::EndMarker(c);
            }
        }
        SymbolInfo::OutOfRange(c)
    }

    fn is_virama(&self, c:u32) -> bool {
        c == self.virama
    }

    fn is_vowel(&self, c:u32) -> bool {
        for i in self.independent_vowels.iter() {
            if c == *i {
                return true;
            }
        }
        false
    }

    fn is_consonant(&self, c:u32) -> bool {
        for i in self.consonants.iter() {
            if c == *i {
                return true;
            }
        }
        false
    }

    fn is_vowel_suffix(&self, c:u32) -> bool {
        for i in self.vowel_suffixes.iter() {
            if c == *i {
                return true;
            }
        }
        false
    }

    fn is_reserved(&self, c:u32) -> bool {
        for i in self.reserved.iter() {
            if c == *i {
                return true;
            }
        }
       false
    }

    fn is_ignored(&self, c:u32) -> bool {
        for i in self.ignored.iter() {
            if c == *i {
                return true;
            }
        }
        false
    }

    fn is_vowel_symbol(&self, c:u32) -> bool {
        for i in self.vowel_signs.iter() {
            if c == *i {
                return true;
            }
        }
        self.is_vowel_suffix(c)
    }

    fn is_digit(&self, c:u32) -> bool {
        for i in self.digits.iter() {
            if c == *i {
                return true;
            }
        }
        false
    }

    fn to_ascii_digit(&self, c:u32) -> u32 {
        if self.is_digit(c) {
            return (c - 0xC66) + 48;
        }
        panic!("Not a digit UC{:x}", c);
    }


    fn is_separator(&self, c:u32) -> bool {
        c < 129 || self.is_reserved(c) || self.is_ignored(c) || self.is_digit(c) || (0x2000..=0x206F).contains(&c) || self.end_of_text.contains(&c)
    }
}

fn read_unicode_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<u32>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.chars().map(|c| c as u32).collect())
}

struct Converter {
    stack : Vec<SymbolInfo>,
    syllables: Vec<Syllable>
}

impl Converter {
    pub fn new() -> Converter {
        Converter { stack: Vec::new(), syllables: Vec::new() }
    }

    pub fn add_code_point(&mut self, symbol:&SymbolInfo, config:&Config) -> Result<(), &str> {
        if self.stack.is_empty() {
            // The followin rules apply here:
            // the symbol can only be a consonant, vowel, digit, out of range or end of text.
            match *symbol {
                SymbolInfo::Vowel(_) | SymbolInfo::Consonant(_) | SymbolInfo::EndMarker(_) => {
                    self.stack.push(*symbol);
                },
                SymbolInfo::OutOfRange(c) => {
                    self.syllables.push(Syllable::Mono(c));
                },
                SymbolInfo::Digit(c) => {
                    self.syllables.push(Syllable::Mono(config.to_ascii_digit(c)));
                },
                SymbolInfo::Ignored(_) => {
                    // ignore
                },
                _ => {
                    return Err("Invalid symbol found in the beginning of the text");
                }
            }
        }
        else {
            let top = self.stack.last().unwrap();
            match *top {
                SymbolInfo::VowelSuffix(_) => {
                    match *symbol {
                        SymbolInfo::VowelSign(_) => {
                            let vowel_sign = self.stack.pop().unwrap();
                            let vowel_suffix = self.stack.pop().unwrap();
                            self.syllables.push(Syllable::Di(vowel_suffix, vowel_sign));
                        },
                        _ => {
                            return Err("Invalid symbol found after a vowel suffix");
                        }
                    }
                },
                SymbolInfo::VowelSign(_) => {
                    match *symbol {
                        SymbolInfo::VowelSuffix(_) => {
                            // ignore
                        },
                        SymbolInfo::Vowel(_) => {
                            let vowel = self.stack.pop().unwrap();
                            let vowel_sign = self.stack.pop().unwrap();
                            self.syllables.push(Syllable::Di(vowel_sign, vowel));
                        },
                        SymbolInfo::Consonant(_) => {
                            let consonant = self.stack.pop().unwrap();
                            let vowel_sign = self.stack.pop().unwrap();
                            self.syllables.push(Syllable::Di(vowel_sign, consonant));
                        },
                        _ => {
                            return Err("Invalid symbol found after a vowel sign");
                        }
                    }
                },
                SymbolInfo::Vowel(_) => {
                    match *symbol {
                        SymbolInfo::VowelSuffix(_) => {
                            let vowel = self.stack.pop().unwrap();
                            let vowel_suffix = self.stack.pop().unwrap();
                            self.syllables.push(Syllable::Di(vowel, vowel_suffix));
                        },
                        SymbolInfo::VowelSign(_) => {
                            let vowel = self.stack.pop().unwrap();
                            let vowel_sign = self.stack.pop().unwrap();
                            self.syllables.push(Syllable::Di(vowel_sign, vowel));
                        },
                        SymbolInfo::Consonant(_) => {
                            let vowel = self.stack.pop().unwrap();
                            let consonant = self.stack.pop().unwrap();
                            self.syllables.push(Syllable::Di(vowel, consonant));
                        },
                        _ => {
                            return Err("Invalid symbol found after a vowel");
                        }
                    }
                },
                SymbolInfo::Consonant(_) => {
                    match *symbol {
                        SymbolInfo::VowelSuffix(_) => {
                            let consonant = self.stack.pop().unwrap();
                            let vowel_suffix = self.stack.pop().unwrap();
                            self.syllables.push(Syllable::Cvc(consonant, vowel_suffix, 0));
                        },
                        SymbolInfo::VowelSign(_) => {
                            let consonant = self.stack.pop().unwrap();
                            let vowel_sign = self.stack.pop().unwrap();
                            self.syllables.push(Syllable::Cvc(consonant, vowel_sign, 0));
                        },
                        SymbolInfo::Vowel(_) => {
                            let consonant = self.stack.pop().unwrap();
                            let vowel = self.stack.pop().unwrap();
                            self.syllables.push(Syllable::Cvc(consonant, vowel, 0));
                        },
                        SymbolInfo::Consonant(_) => {
                            let consonant2 = self.stack.pop().unwrap();
                            let consonant1 = self.stack.pop().unwrap();
                            self.syllables.push(Syllable::Cc(consonant1, consonant2));
                        },
                        _ => {
                            return Err("Invalid symbol found after a consonant");
                        }
                    }
                },
                SymbolInfo::Virama(_) => {
                    match *symbol {
                        SymbolInfo::VowelSuffix(_) => {
                            let virama = self.stack.pop().unwrap();
                            let vowel_suffix = self.stack.pop().unwrap();
                            self.syllables.push(Syllable::Cvc(virama, vowel_suffix, 0));
                        },
                        SymbolInfo::VowelSign(_) => {
                            let virama = self.stack.pop().unwrap();
                            let vowel_sign = self.stack.pop().unwrap();
                            self.syllables.push(Syllable::Cvc(virama, vowel_sign, 0));
                        },
                        SymbolInfo::Vowel(_) => {
                            let virama = self.stack.pop().unwrap();
                            let vowel = self.stack.pop().unwrap();
                            self.syllables.push(Syllable::Cvc(virama, vowel, 0));
                        },
                        SymbolInfo::Consonant(_) => {
                            let virama = self.stack.pop().unwrap();
                            let consonant = self.stack.pop().unwrap();
                            self.syllables.push(Syllable::Cvc(virama, consonant, 0));
                        },
                        SymbolInfo::Virama(_) => {
                            let virama2 = self.stack.pop().unwrap();
                            let virama1 = self.stack.pop().unwrap();
                            self.syllables.push(Syllable::Cc(virama1, virama2));
                        },
                        _ => {
                            return Err("Invalid symbol found after a virama");
                        }
                    }
                },
                SymbolInfo::Ignored(_) => {
                    // ignore
                },
                SymbolInfo::Digit(_) => {
                    match *symbol {
                        SymbolInfo::VowelSuffix(_) => {
                            let digit = self.stack.pop().unwrap();
                            let vowel_suffix = self.stack.pop().unwrap();
                            self.syllables.push(Syllable::Cvc(digit, vowel_suffix, 0));
                        },
                        SymbolInfo::VowelSign(_) => {
                            let digit = self.stack.pop().unwrap();
                            let vowel_sign = self.stack.pop().unwrap();
                            self.syllables.push(Syllable::Cvc(digit, vowel_sign, 0));
                        },
                        SymbolInfo::Vowel(_) => {
                            let digit = self.stack.pop().unwrap();
                            let vowel = self.stack.pop().unwrap();
                            self.syllables.push(Syllable::Cvc(digit, vowel, 0));
                        },
                        SymbolInfo::Consonant(_) => {
                            let digit = self.stack.pop().unwrap();
                            let consonant = self.stack.pop().unwrap();
                            self.syllables.push(Syllable::Cvc(digit, consonant, 0));
                        },
                        SymbolInfo::Digit(_) => {
                            let digit2 = self.stack.pop().unwrap();
                            let digit1 = self.stack.pop().unwrap();
                            self.syllables.push(Syllable::Cc(digit1, digit2));
                        },
                        _ => {
                            return Err("Invalid symbol found after a digit");
                        }
                    }
                },
                SymbolInfo::OutOfRange(_) => {
                    // ignore
                }
                SymbolInfo::EndMarker(_) => {
                    // ignore
                }
            }

        }
        Ok(())
    }
}
// reads the next token into syllabary
// take the first character.
// if it is a consonant,
//        read the next character. If is a vowel_symbol, then create a diphthong.
//                                 If it is a virama, then read the next character. If it is a
//                                 separator, then a halanth diphong is created.
//                                 if it is a consonant, then check the next character. If it is a vowel_symbol, then create a triphthong.
//                                 if it is a separator, then create a halanth triphthong.
//  if it is a vowel, then check the next character. if it is a vowel_suffix, then create a monophthong.
//                    else create a monophthong with the vowel.
//  if it is a digit, then add ascii value to the syllabary.
//  if it is a separator, then add the ascii value to the syllabary.
//  if it is reserved, panic
//  if it is ignored, ignore
fn read_next_syllable(contents:&[u32], syllabary:&SyllableMapping, config:&Config, line_no:usize, col_no:usize) -> (usize, u32) {

    if contents.is_empty() {
        return (0, 0);
    }

    let first = contents[0];
    if config.is_reserved(first) {
        println!("Reserved character found UC{:x} at {}:{}", first, line_no, col_no);
        return (1, 0);
    }
    if first < 128 {
        return (1, syllabary.syllable_code(Syllable::Mono(first), config));
    }
    if config.is_ignored(first) {
        return (1, 1);
    }
    if config.is_digit(first) {
        return (1, syllabary.syllable_code(Syllable::Mono(config.to_ascii_digit(first)), config));
    }
    if config.is_separator(first) {
        return (1, syllabary.syllable_code(Syllable::Mono(first), config));
    }
    if config.is_vowel_symbol(first) {
        //println!("Vowel symbol found U{:x} at line {}:{}", first, line_no, col_no);
        return (1,0);
        
    }
    if config.is_vowel_suffix(first) {
        panic!("Vowel suffix found U{:x} at line {}:{}", first, line_no, col_no);
    }

    if config.is_vowel(first) {
        if contents.len() == 1 {
            return (1, syllabary.syllable_code(Syllable::Mono(first), config));
        }
        let second = contents[1];
        if config.is_vowel_suffix(second) {
            return (2, syllabary.syllable_code(Syllable::Di(first, second), config));
        }
        return (1, syllabary.syllable_code(Syllable::Mono(first), config));
    }

    if config.is_consonant(first) {
        if contents.len() == 1 {
            return (1, syllabary.syllable_code(Syllable::Mono(first), config));
        }
        let second = contents[1];
        if config.is_virama(second) {
            if contents.len() == 2 {
                return (2, syllabary.syllable_code(Syllable::Di(first, second), config));
            }
            let third = contents[2];
            if config.is_separator(third) {
                return (2, syllabary.syllable_code(Syllable::Di(first, second), config));
            }
            if config.is_consonant(third) {
                if contents.len() == 3 {
                    return (3, syllabary.syllable_code(Syllable::Cc(first, third), config));
                }
                let fourth = contents[3];
                if config.is_vowel_symbol(fourth) {
                    if contents.len() == 4 {
                        return (4, syllabary.syllable_code(Syllable::Cvc(first, third, fourth), config));
                    }
                    let fifth = contents[4];
                    if config.is_vowel_suffix(fifth) {
                        return (5, syllabary.syllable_code(Syllable::Cvvc(first, third, fourth, fifth), config));
                    }

                    return (4, syllabary.syllable_code(Syllable::Cvc(first, third, fourth), config));
                }
                return (3, syllabary.syllable_code(Syllable::Cc(first, third), config));
            }
            if config.is_vowel_symbol(third) {
                //println!("Vowel symbol found U0{:x}{:x}{:x} following a virama at {}:{} ", first, second, third, line_no, col_no);
                return (3, 0);
            }
            return (3, syllabary.syllable_code(Syllable::Cc(first, second), config));
        }
        if config.is_vowel_symbol(second) {
            if contents.len() == 2 {
                return (2, syllabary.syllable_code(Syllable::Di(first, second), config));
            }
            let third = contents[2];
            if config.is_vowel_suffix(third) {
                return (3, syllabary.syllable_code(Syllable::Cvv(first, second, third), config));
            }

            return (2, syllabary.syllable_code(Syllable::Di(first, second), config));
        }
    }

    (1, syllabary.syllable_code(Syllable::Mono(first), config))
}

#[derive(Serialize, Deserialize)]
struct SyllableToken {
    syllable : Syllable,
    token : u32,
}

#[derive(Serialize, Deserialize)]
struct SyllableMappingFile {
    syllables : Vec<SyllableToken>,
    maximum   : u32,
}

impl SyllableMappingFile {

    fn read_from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let contents = read_unicode_file(path)?;
        let syllabary : SyllableMappingFile = serde_json::from_str(&contents.iter().map(|&c| c as u8 as char).collect::<String>()).unwrap();
        Ok(syllabary)
    }

    fn to_syllable_mapping(&self, config:&Config) -> SyllableMapping {
        let mut tokens = Vec::new();
        tokens.resize((self.maximum + 1) as usize, Syllable::Mono(0));
        let mut table = HashMap::new();
        for syllable in self.syllables.iter() {
            tokens[syllable.token as usize] = syllable.syllable;
            table.insert(syllable.syllable, syllable.token);
        }
        let mut sm = SyllableMapping { syllable_token : table, token_syllable : tokens };
        sm.add_syllable(Syllable::Meta(config.unknown));
        for c in config.end_of_text.iter() {
            sm.add_syllable(Syllable::Meta(*c));
        }

        sm
    }
}

#[derive(Serialize, Deserialize)]
struct EncodedText {
    size : usize,
    lines : Vec<Vec<u32>>,
}

fn encode_contents(contents:&[u32], syllabary:&SyllableMapping, config:&Config, insert_eof:bool) -> Vec<u32> {
    let mut encoded = Vec::new();
    if insert_eof {
        encoded.push(syllabary.syllable_code(Syllable::Meta(*config.end_of_text.last().unwrap()), config));
    }
    let mut i = 0;
    let mut line_no = 1;
    let mut col_no = 1;
    while i < contents.len() {
        while i < contents.len() {
            let (n, token) = read_next_syllable(&contents[i..], syllabary, config, line_no, col_no);
            if token != 0 {
                encoded.push(token);
                if token == 0xA {
                    line_no += 1;
                    col_no = 1;
                } else {
                    col_no += n;
                }
            }
            i += n;
        }
    }
    encoded
}

fn decode_contents(encoded:&[u32], syllabary:&SyllableMapping, config:&Config) -> String {
    let mut text = String::new();
    for token in encoded.iter() {
        let syllable = syllabary.get_syllable(*token).unwrap();
        syllable.append_char(&mut text, config);
    }
    text
}

#[pyclass]
struct Tokenizer{
    syllabary : SyllableMapping,
    config : Config,
}

impl Tokenizer {
    fn new_telugu(vocab_file:String) -> Self {
        let config = Config::new_telugu();
        let syllabary = SyllableMappingFile::read_from_file(&vocab_file).unwrap().to_syllable_mapping(&config);
        Tokenizer { syllabary, config }
    }

    fn new_devnagari(vocab_file:String) -> Self {
        let config = Config::new_devnagari();
        let syllabary = SyllableMappingFile::read_from_file(&vocab_file).unwrap().to_syllable_mapping(&config);
        Tokenizer { syllabary, config }
    }
}
/// Performs Tokenization for telugu texts written in Brahmi Script.
/// It relies on telugu vocabulary json file.
#[pymethods]
impl Tokenizer {
    #[new]
    fn new(script_name:String, vocab_file:String) -> Self {
        match script_name.as_str() {
            "telugu" => Tokenizer::new_telugu(vocab_file),
            "devnagari" => Tokenizer::new_devnagari(vocab_file),
            _ => panic!("Unknown script {}", script_name),
        }
    }

    fn encode(&self, text:String) -> Vec<u32> {
        let contents = text.chars().map(|c| c as u32).collect::<Vec<u32>>();
        encode_contents(&contents, &self.syllabary, &self.config, false)
    }

    fn decode(&self, encoded:Vec<u32>) -> String {
        decode_contents(&encoded, &self.syllabary, &self.config)
    }

    fn encode_file(&self, input_file:String) -> PyResult<Vec<u32>> {
        let contents = read_unicode_file(&input_file);
        if let Err(msg) = contents {
            return Err(PyValueError::new_err(msg));
        }
        let contents = contents.unwrap();
        let encoded = encode_contents(&contents, &self.syllabary, &self.config, true);
        Ok(encoded)
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn brahmi_script(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Tokenizer>()?;
    Ok(())
}
