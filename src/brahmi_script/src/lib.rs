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

#[derive(Debug,Clone,PartialEq,Eq, Hash, Serialize, Deserialize)]
enum Syllable {
    Mono(u32),
    Multi(Vec<u32>),
    Double(u32, u32),
    Triple(u32, u32, u32),
    Quad(u32, u32, u32, u32),
    Penta(u32, u32, u32, u32, u32),
    Hexa(u32, u32, u32, u32, u32, u32),
    Septa(u32, u32, u32, u32, u32, u32, u32),
    /// Meta character
    Meta(u32),
}

impl Syllable {

    fn append_char(&self, str:&mut String, config:&Config) {
        match *self {
            Syllable::Mono(c) => {
                str.push(std::char::from_u32(c).unwrap());
            },
            Syllable::Double(c1, c2) => {
                str.push(std::char::from_u32(c1).unwrap());
                str.push(std::char::from_u32(c2).unwrap());
            },
            Syllable::Triple(c1, c2, c3) => {
                str.push(std::char::from_u32(c1).unwrap());
                str.push(std::char::from_u32(c2).unwrap());
                str.push(std::char::from_u32(c3).unwrap());
            },
            Syllable::Quad(c1, c2, c3, c4) => {
                str.push(std::char::from_u32(c1).unwrap());
                str.push(std::char::from_u32(c2).unwrap());
                str.push(std::char::from_u32(c3).unwrap());
                str.push(std::char::from_u32(c4).unwrap());
            },
            Syllable::Penta(c1, c2, c3, c4, c5) => {
                str.push(std::char::from_u32(c1).unwrap());
                str.push(std::char::from_u32(c2).unwrap());
                str.push(std::char::from_u32(c3).unwrap());
                str.push(std::char::from_u32(c4).unwrap());
                str.push(std::char::from_u32(c5).unwrap());
            },
            Syllable::Hexa(c1, c2, c3, c4, c5, c6) => {
                str.push(std::char::from_u32(c1).unwrap());
                str.push(std::char::from_u32(c2).unwrap());
                str.push(std::char::from_u32(c3).unwrap());
                str.push(std::char::from_u32(c4).unwrap());
                str.push(std::char::from_u32(c5).unwrap());
                str.push(std::char::from_u32(c6).unwrap());
            },
            Syllable::Multi(ref v) => {
                for c in v.iter() {
                    str.push(std::char::from_u32(*c).unwrap());
                }
            },
            Syllable::Septa(c1, c2, c3, c4, c5, c6, c7) => {
                str.push(std::char::from_u32(c1).unwrap());
                str.push(std::char::from_u32(c2).unwrap());
                str.push(std::char::from_u32(c3).unwrap());
                str.push(std::char::from_u32(c4).unwrap());
                str.push(std::char::from_u32(c5).unwrap());
                str.push(std::char::from_u32(c6).unwrap());
                str.push(std::char::from_u32(c7).unwrap());
            },
            Syllable::Meta(c) => {
                str.push(std::char::from_u32(c).unwrap());
            }
        }
    }
}

impl fmt::Display for Syllable {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Syllable::Mono(c) => write!(f, "Mono(U{:x}, {})", c, std::char::from_u32(c).unwrap()),
            Syllable::Double(c1, c2) => write!(f, "Double(U{:x}, U{:x}, {}{})", c1, c2, std::char::from_u32(c1).unwrap(), std::char::from_u32(c2).unwrap()),
            Syllable::Triple(c1, c2, c3) => write!(f, "Triple(U{:x}, U{:x}, U{:x}, {}{}{})", c1, c2, c3, std::char::from_u32(c1).unwrap(), std::char::from_u32(c2).unwrap(), std::char::from_u32(c3).unwrap()),
            Syllable::Quad(c1, c2, c3, c4) => write!(f, "Quad(U{:x}, U{:x}, U{:x}, U{:x}, {}{}{}{})", c1, c2, c3, c4, std::char::from_u32(c1).unwrap(), std::char::from_u32(c2).unwrap(), std::char::from_u32(c3).unwrap(), std::char::from_u32(c4).unwrap()),
            Syllable::Penta(c1, c2, c3, c4,c5) => write!(f, "Penta(U{:x}, U{:x}, U{:x}, U{:x}, U{:x}, {}{}{}{}{})", c1, c2, c3, c4,c5, std::char::from_u32(c1).unwrap(), std::char::from_u32(c2).unwrap(), std::char::from_u32(c3).unwrap(), std::char::from_u32(c4).unwrap(),   std::char::from_u32(c5).unwrap()),
            Syllable::Hexa(c1, c2, c3, c4, c5, c6) => write!(f, "Hexa(U{:x}, U{:x}, U{:x}, U{:x}, U{:x}, U{:x} {}{}{}{}{}{})", c1, c2, c3, c4, c5, c6, std::char::from_u32(c1).unwrap(), std::char::from_u32(c2).unwrap(), std::char::from_u32(c3).unwrap(), std::char::from_u32(c4).unwrap(), char::from_u32(c5).unwrap(), std::char::from_u32(c6).unwrap()),
            Syllable::Septa(c1, c2, c3, c4, c5, c6, c7) => write!(f, "Septa(U{:x}, U{:x}, U{:x}, U{:x}, U{:x}, U{:x}, U{:x}, {}{}{}{}{}{}{})", c1, c2, c3, c4, c5, c6, c7, std::char::from_u32(c1).unwrap(), std::char::from_u32(c2).unwrap(), std::char::from_u32(c3).unwrap(), std::char::from_u32(c4).unwrap(), std::char::from_u32(c5).unwrap(), std::char::from_u32(c6).unwrap(), std::char::from_u32(c7).unwrap()),
            Syllable::Multi(ref v) => {
                write!(f, "Multi(")?;
                for c in v.iter() {
                    write!(f, "U{:x}, ", c)?;
                }
                write!(f, ")")
            },
            Syllable::Meta(c) => write!(f, "Meta(U{:x}, {})", c, std::char::from_u32(c).unwrap()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SyllableMapping {
    syllable_token : HashMap<Syllable, u32>,
    token_syllable : Vec<Syllable>,
}

impl SyllableMapping {

    // Creates a new syllable mapping from a configuration.
    fn new(config:&Config) -> Self {
        let mut syllable_token = HashMap::new();
        let mut token_syllable = Vec::new();
        for i in 0..128 {
            syllable_token.insert(Syllable::Mono(i), i);
            token_syllable.push(Syllable::Mono(i));
        }
        syllable_token.insert(Syllable::Meta(config.unknown), 128);
        token_syllable.push(Syllable::Meta(config.unknown));
        let mut i = 129;
        for c in config.end_of_text.iter() {
            syllable_token.insert(Syllable::Meta(*c), i);
            token_syllable.push(Syllable::Meta(*c));
            i += 1;
        }
        SyllableMapping { syllable_token, token_syllable }
    }

    // Adds a syllable to the syllabary and returns the encoded token.
    fn syllable_code(&self, syllable : Syllable, config:&Config) -> u32 {
        if let Some(&token) = self.syllable_token.get(&syllable) {
            return token;
        }
        config.unknown
    }

    fn get_syllable(&self, token : u32) -> Option<Syllable> {
        if token < self.token_syllable.len() as u32 {
            Some(self.token_syllable[token as usize].clone())
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
        self.syllable_token.insert(syllable.clone(), token);
        self.token_syllable.push(syllable);
    }

    fn to_syllable_mapping_file(&self) -> SyllableMappingFile {
        let mut syllables = Vec::new();
        let mut maximum = 0;
        for (s, t) in self.syllable_token.iter() {
            syllables.push(SyllableToken { syllable : s.clone(), token : *t });
            if *t > maximum {
                maximum = *t;
            }
        }
        SyllableMappingFile { syllables , maximum }
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

impl SymbolInfo {
    fn get_u32(&self) -> u32 {
        match *self {
            SymbolInfo::VowelSuffix(i) => i,
            SymbolInfo::VowelSign(i)   => i,
            SymbolInfo::Vowel(i)       => i,
            SymbolInfo::Consonant(i)   => i,
            SymbolInfo::Virama(i)      => i,
            SymbolInfo::Ignored(i)     => i,
            SymbolInfo::Digit(i)       => i,
            SymbolInfo::EndMarker(i)   => i,
            SymbolInfo::OutOfRange(i)  => i
        }
    }
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

        let mut ascii_digit = 48;
        for i in self.digits.iter() {
            if c == *i {
                return SymbolInfo::Digit(ascii_digit);
            }
            ascii_digit += 1;
        }
        for eot in self.end_of_text.iter() {
            if c == *eot {
                return SymbolInfo::EndMarker(c);
            }
        }
        SymbolInfo::OutOfRange(c)
    }

    fn virama(&self) -> u32 {
        self.virama
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

    fn clear_stack(&mut self) -> bool {
        if self.stack.is_empty() {
            return true;
        }
        if self.stack.len() == 1 {
            let top = self.stack.pop().unwrap().get_u32();
            self.syllables.push(Syllable::Mono(top));
            return true;
        }
        if self.stack.len() <= 8 {
            let mut v = Vec::new();
            for s in self.stack.iter() {
                v.push(s.get_u32());
            }
            self.syllables.push(Syllable::Multi(v));
            self.stack.clear();
            return true;
        }
        println!("Stack size = {}", self.stack.len());
        let mut string = String::new();
        for s in self.stack.iter() {
            string.push(std::char::from_u32(s.get_u32()).unwrap());
        }
        println!("Too big a stack to clear = {} ({})", string, self.stack.len());
        self.stack.clear();
        false
    }

    pub fn finish(&mut self, virama:u32) -> bool {
        self.clear_stack()
    }

    pub fn add_code_point(&mut self, symbol:&SymbolInfo, virama:u32) -> Result<(), String> {
        if let SymbolInfo::Consonant(_) = symbol {
            if self.stack.is_empty() {
                self.stack.push(symbol.clone());
                return Ok(());
            }
            if self.stack.last().unwrap().get_u32() == virama {
                self.stack.push(symbol.clone());
                return Ok(());
            }
            if self.clear_stack() {
                self.stack.push(*symbol);
                return Ok(());
            } else {
                return Err(format!("error in text before consonant {}", std::char::from_u32(symbol.get_u32()).unwrap()));
            }
        }
        if let SymbolInfo::Vowel(_) = symbol {
            if self.clear_stack() {
                self.stack.push(*symbol);
                return Ok(());
            } else {
                return Err(format!("error in text before vowel {}", std::char::from_u32(symbol.get_u32()).unwrap()));
            }
        }
        if let SymbolInfo::VowelSign(_) = symbol {
            self.stack.push(*symbol);
            return Ok(());
        }
        if let SymbolInfo::VowelSuffix(v) = symbol {
            if self.stack.is_empty() {
                return Err(format!("unexpected {}", std::char::from_u32(*v).unwrap()));
            }
            self.stack.push(*symbol);
            return Ok(());
        }
        if let SymbolInfo::Virama(v) = symbol {
            if self.stack.is_empty() {
                return Err(format!("unexpected virama {}", std::char::from_u32(*v).unwrap()));
            }
            if self.stack.len() != 1 {
               // return Err(format!("unexpected virama {} when stack size is {}", std::char::from_u32(*v).unwrap(), self.stack.len()));
            }
            self.stack.push(*symbol);
            return Ok(());
        }
        if let SymbolInfo::Digit(d) = symbol {
            if self.clear_stack() {
                self.syllables.push(Syllable::Mono(*d));
                return Ok(());
            } else {
                return Err(format!("error in text before digit {}", std::char::from_u32(*d).unwrap()));
            }
        }
        if let SymbolInfo::EndMarker(v) = symbol {
            if self.clear_stack() {
                self.syllables.push(Syllable::Mono(*v));
                return Ok(());
            }
            return Err("error in text before end marker".to_string());
        }
        if let SymbolInfo::OutOfRange(r) = symbol {
            if self.clear_stack() {
                self.syllables.push(Syllable::Mono(*r));
                return Ok(());
            }
            return Err(format!("error in text before out of range {}", std::char::from_u32(*r).unwrap()));
        }
        Ok(())
    }
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
        if self.syllables.is_empty() {
            return SyllableMapping::new(config);
        }
        let mut tokens = Vec::new();
        tokens.resize((self.maximum + 1) as usize, Syllable::Mono(0));
        let mut table = HashMap::new();
        for syllable in self.syllables.iter() {
            tokens[syllable.token as usize] = syllable.syllable.clone();
            table.insert(syllable.syllable.clone(), syllable.token);
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

fn collect_vocab(contents:&[u32], syllabary:&mut SyllableMapping,config:&Config) -> bool {
    let mut result = true;
    let mut converter = Converter::new();
    let mut i = 0;
    let mut line_no = 1;
    let mut col_no = 1;
    while i < contents.len() {
        let symbol_info = config.to_symbol_info(contents[i]);
        if let Err(msg) = converter.add_code_point(&symbol_info, config.virama()) {
            println!("{} at {}:{}. Current char = {}", msg, line_no, col_no, std::char::from_u32(contents[i]).unwrap());
            result = false;
        }
        if symbol_info.get_u32() == 0xA {
            line_no += 1;
            col_no = 1;
        } else {
            col_no += 1;
        }
        i += 1;
    }
    result = converter.finish(config.virama()) && result;
    for s in converter.syllables.iter() {
        syllabary.add_syllable(s.clone());
    }
    result
}
fn encode_contents(contents:&[u32], syllabary:&SyllableMapping, config:&Config, insert_eof:bool) -> Vec<u32> {
    let mut encoded = Vec::new();
    if insert_eof {
        encoded.push(syllabary.syllable_code(Syllable::Meta(*config.end_of_text.last().unwrap()), config));
    }
    let mut converter = Converter::new();
    let mut i = 0;
    let mut line_no = 1;
    let mut col_no = 1;
    while i < contents.len() {
        let symbol_info = config.to_symbol_info(contents[i]);
        if let Err(msg) = converter.add_code_point(&symbol_info, config.virama()) {
            println!("{} at {}:{}. Current char = {}", msg, line_no, col_no, std::char::from_u32(contents[i]).unwrap());
        }
        if symbol_info.get_u32() == 0xA {
            line_no += 1;
            col_no = 1;
        } else {
            col_no += 1;
        }
        i += 1;
    }
    converter.finish(config.virama());
    for s in converter.syllables.iter() {
        encoded.push(syllabary.syllable_code(s.clone(), config));
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

#[cfg(test)]
mod telugu_tests{
    use super::*;

    #[test]
    fn sthothramulu() {
        let test_word = "స్తోత్రములు";
        let config = Config::new_telugu();
        let mut converter = Converter::new();
        for chr in test_word.chars() {
            let c:u32 = chr.into(); 
            let symbol_info = config.to_symbol_info(c);
            converter.add_code_point(&symbol_info, config.virama()).unwrap();
        }
        assert!(converter.finish(config.virama()));
        let mut round_trip = String::new();
        for s in converter.syllables.iter() {
            s.append_char(&mut round_trip, &config);
        }
        assert_eq!(4, converter.syllables.len());
        assert_eq!(test_word, round_trip);
    }

    #[test]
    fn puranalu() {
        let test_word = "పురాణాలు";
        let config = Config::new_telugu();
        let mut converter = Converter::new();
        for chr in test_word.chars() {
            let c:u32 = chr.into(); 
            let symbol_info = config.to_symbol_info(c);
            converter.add_code_point(&symbol_info, config.virama()).unwrap();
        }
        assert!(converter.finish(config.virama()));
        let mut round_trip = String::new();
        for s in converter.syllables.iter() {
            s.append_char(&mut round_trip, &config);
        }
        assert_eq!(4, converter.syllables.len());
        assert_eq!(test_word, round_trip);
    }

    #[test]
    fn kavithvamu() {
        let test_word = "కవిత్వము";
        let config = Config::new_telugu();
        let mut converter = Converter::new();
        for chr in test_word.chars() {
            let c:u32 = chr.into(); 
            let symbol_info = config.to_symbol_info(c);
            converter.add_code_point(&symbol_info, config.virama()).unwrap();
        }
        assert!(converter.finish(config.virama()));
        let mut round_trip = String::new();
        for s in converter.syllables.iter() {
            s.append_char(&mut round_trip, &config);
        }
        assert_eq!(4, converter.syllables.len());
        assert_eq!(test_word, round_trip);
    }
    #[test]
    fn paaschatyavidwamsulache() {
        let test_word = "పాశ్చాత్యవిద్యాంసులచే";
        let config = Config::new_telugu();
        let mut converter = Converter::new();
        for chr in test_word.chars() {
            let c:u32 = chr.into(); 
            let symbol_info = config.to_symbol_info(c);
            converter.add_code_point(&symbol_info, config.virama()).unwrap();
        }
        assert!(converter.finish(config.virama()));
        let mut round_trip = String::new();
        for s in converter.syllables.iter() {
            s.append_char(&mut round_trip, &config);
        }
        assert_eq!(8, converter.syllables.len());
        assert_eq!(test_word, round_trip);
    }
    #[test]
    fn granthamu() {
        let test_word = "గ్రంధము";
        let config = Config::new_telugu();
        let mut converter = Converter::new();
        for chr in test_word.chars() {
            let c:u32 = chr.into(); 
            let symbol_info = config.to_symbol_info(c);
            converter.add_code_point(&symbol_info, config.virama()).unwrap();
        }
        assert!(converter.finish(config.virama()));
        let mut round_trip = String::new();
        for s in converter.syllables.iter() {
            s.append_char(&mut round_trip, &config);
        }
        assert_eq!(3, converter.syllables.len());
        assert_eq!(test_word, round_trip);
    }

    #[test]
    fn raashtram() {
        let test_word = "రాష్ట్రం";
        let config = Config::new_telugu();
        let mut converter = Converter::new();
        for chr in test_word.chars() {
            let c:u32 = chr.into(); 
            let symbol_info = config.to_symbol_info(c);
            converter.add_code_point(&symbol_info, config.virama()).unwrap();
        }
        assert!(converter.finish(config.virama()));
        let mut round_trip = String::new();
        for s in converter.syllables.iter() {
            s.append_char(&mut round_trip, &config);
        }
        assert_eq!(2, converter.syllables.len());
        assert_eq!(test_word, round_trip);
    }
    #[test]
    fn bhattaacharya() {
        let test_word = "భట్టాచార్య";
        let config = Config::new_telugu();
        let mut converter = Converter::new();
        for chr in test_word.chars() {
            let c:u32 = chr.into(); 
            let symbol_info = config.to_symbol_info(c);
            converter.add_code_point(&symbol_info, config.virama()).unwrap();
        }
        assert!(converter.finish(config.virama()));
        let mut round_trip = String::new();
        for s in converter.syllables.iter() {
            s.append_char(&mut round_trip, &config);
        }
        assert_eq!(4, converter.syllables.len());
        assert_eq!(test_word, round_trip);
    }
    #[test]
    fn vignyaana_saasthram() {
        let test_word = "విజ్ఞానశాస్త్ర్రం";
        let config = Config::new_telugu();
        let mut converter = Converter::new();
        for chr in test_word.chars() {
            let c:u32 = chr.into(); 
            let symbol_info = config.to_symbol_info(c);
            converter.add_code_point(&symbol_info, config.virama()).unwrap();
        }
        assert!(converter.finish(config.virama()));
        let mut round_trip = String::new();
        for s in converter.syllables.iter() {
            s.append_char(&mut round_trip, &config);
        }
        assert_eq!(5, converter.syllables.len());
        assert_eq!(test_word, round_trip);
    }
}

#[cfg(test)]
mod hindi_tests{
    use super::*;

    #[test]
    fn khaak() {
        let test_word = "खाक़";
        let config = Config::new_devnagari();
        let mut converter = Converter::new();
        for chr in test_word.chars() {
            let c:u32 = chr.into(); 
            let symbol_info = config.to_symbol_info(c);
            converter.add_code_point(&symbol_info, config.virama()).unwrap();
        }
        converter.finish(config.virama());
        let mut round_trip = String::new();
        for s in converter.syllables.iter() {
            s.append_char(&mut round_trip, &config);
        }
        assert_eq!(2, converter.syllables.len());
        assert_eq!(test_word, round_trip);
    }

    #[test]
    fn koee() {
        let test_word = "कोई";
        let config = Config::new_devnagari();
        let mut converter = Converter::new();
        for chr in test_word.chars() {
            let c:u32 = chr.into(); 
            let symbol_info = config.to_symbol_info(c);
            converter.add_code_point(&symbol_info, config.virama()).unwrap();
        }
        converter.finish(config.virama());
        let mut round_trip = String::new();
        for s in converter.syllables.iter() {
            s.append_char(&mut round_trip, &config);
            println!("{}", s);
        }
        assert_eq!(2, converter.syllables.len());
        assert_eq!(test_word, round_trip);
    }

    #[test]
    fn fizaaon() {
        let test_word = "फ़िज़ाओं";
        let config = Config::new_devnagari();
        let mut converter = Converter::new();
        for chr in test_word.chars() {
            let c:u32 = chr.into(); 
            let symbol_info = config.to_symbol_info(c);
            converter.add_code_point(&symbol_info, config.virama()).unwrap();
        }
        converter.finish(config.virama());
        let mut round_trip = String::new();
        for s in converter.syllables.iter() {
            s.append_char(&mut round_trip, &config);
        }
        assert_eq!(3, converter.syllables.len());
        assert_eq!(test_word, round_trip);
    }
    #[test]
    fn vyarth_gavaaye() {
        let test_word = "व्यर्थ गवाये";
        let config = Config::new_devnagari();
        let mut converter = Converter::new();
        for chr in test_word.chars() {
            let c:u32 = chr.into(); 
            let symbol_info = config.to_symbol_info(c);
            converter.add_code_point(&symbol_info, config.virama()).unwrap();
        }
        converter.finish(config.virama());
        let mut round_trip = String::new();
        for s in converter.syllables.iter() {
            s.append_char(&mut round_trip, &config);
        }
        assert_eq!(6, converter.syllables.len());
        assert_eq!(test_word, round_trip);
    }
    #[test]
    fn jaanenadoongi() {
        let test_word = "जाने न दूँगी";
        let config = Config::new_devnagari();
        let mut converter = Converter::new();
        for chr in test_word.chars() {
            let c:u32 = chr.into(); 
            let symbol_info = config.to_symbol_info(c);
            converter.add_code_point(&symbol_info, config.virama()).unwrap();
        }
        converter.finish(config.virama());
        let mut round_trip = String::new();
        for s in converter.syllables.iter() {
            s.append_char(&mut round_trip, &config);
        }
        assert_eq!(7, converter.syllables.len());
        assert_eq!(test_word, round_trip);
    }
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

    fn collect_vocab(&mut self, input_file:String) -> PyResult<bool> {
        let contents = read_unicode_file(&input_file);
        if let Err(msg) = contents {
            return Err(PyValueError::new_err(msg));
        }
        let contents = contents.unwrap();
        let result = collect_vocab(&contents, &mut self.syllabary, &self.config);
        Ok(result)
    }

    fn write_vocab_file(&self, output_file:String) -> PyResult<()> {
        let syllable_mapping = self.syllabary.to_syllable_mapping_file();
        let json = serde_json::to_string(&syllable_mapping).unwrap();
        let mut file = File::create(output_file).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        Ok(())
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn brahmi_script(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Tokenizer>()?;
    Ok(())
}
