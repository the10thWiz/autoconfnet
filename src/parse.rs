use crate::file::File;
use crate::interface::*;
use crate::ip::*;
use std::collections::HashMap;
use std::ops::*;

#[derive(Debug, Clone)]
enum Value {
    Simple(String, bool),
    Selected(String, Vec<String>, bool),
    Range(isize, Range<isize>, bool),
    IP(IPv4, bool),
    Interface(Iface, bool),
    List(Vec<HashMap<String, Value>>),
}

pub struct Conf {
    vals: HashMap<String, Value>,
    conf: Vec<String>,
}

impl Conf {
    fn parse_line(mut tmp: &str, ret: &mut HashMap<String, Value>) {
        // `?\` is an optional command, ignored for now
        if tmp.starts_with("?\\") {
            tmp = &tmp[2..];
        }
        for w in tmp.split_whitespace() {
            let word = w.trim_matches('"');
            let optional = if word.starts_with("$") {
                // `$` is a required parameter
                false
            } else if word.starts_with("?") {
                // `?` is an optional parameter
                true
            } else {
                continue;
            };
            let mut parts = word
                .split(|c| c == '$' || c == '?' || c == '{' || c == '}')
                .filter(|&s| s != "");
            let name = parts.next().unwrap().split("-").next().unwrap().to_string();
            if let Some(type_name) = parts.next() {
                // `{}` specifies that the parameter only accepts the values listed
                //   Only for sanity checks and error checking. Not needed, but nice to have
                //   Comma seperated for word vals, rust range syntax for nums
                //   \ip: anything that starts with a \ is a special type (e.g. ip, mask, interface)
                //   $ip-mask refers to the mask part of the ip param, -ends key parse
                if type_name.starts_with("\\") {
                    match type_name {
                        "\\ip" | "\\ipslash" => {
                            ret.insert(name, Value::IP(IPv4::default(), optional));
                        }
                        "\\interface" => {
                            ret.insert(name, Value::Interface(Iface::default(), optional));
                        }
                        "\\mac" => {
                            ret.insert(name, Value::Simple(String::default(), optional));
                        }
                        _ => panic!("{} isn't a valid type", type_name),
                    }
                } else if type_name.contains("..") {
                    let mut range_parts = type_name.split("..");
                    let start = range_parts
                        .next()
                        .expect("Num Error")
                        .parse()
                        .expect("Num Error");
                    let end = range_parts
                        .next()
                        .expect("Num Error")
                        .parse()
                        .expect("Num Error");
                    ret.insert(name, Value::Range(-1, start..end, optional));
                } else {
                    ret.insert(
                        name,
                        Value::Selected(
                            String::default(),
                            type_name.split(",").map(|s| s.to_string()).collect(),
                            optional,
                        ),
                    );
                }
            } else {
                ret.insert(name, Value::Simple(String::default(), optional));
            }
        }
    }
    fn parse_pattern(i: &mut std::slice::Iter<String>, ret: &mut HashMap<String, Value>) {
        // let mut i = v.iter();
        while let Some(s) = i.next() {
            let tmp: &str = s.trim();
            // `!` is ignored, excpet when parsing for loops
            if tmp.starts_with("!") || tmp.starts_with("#") {
                if tmp.contains("! for $") || tmp.contains("# for $") {
                    let name = tmp[7..].split_whitespace().next().expect("").to_string();
                    let mut hash = HashMap::new();
                    Self::parse_pattern(i, &mut hash);
                    ret.insert(name, Value::List(vec![hash]));
                } else if tmp.contains("! end for")
                    || tmp.contains("! end conf")
                    || tmp.contains("# end for")
                    || tmp.contains("# end conf")
                {
                    return;
                }
            } else {
                Self::parse_line(tmp, ret);
            }
        }
    }
    pub fn parse(file: &str) -> Self {
        let v: Vec<String> = File::read(file).expect("File ran into issue").collect();
        let mut h = HashMap::new();
        Self::parse_pattern(&mut v.iter(), &mut h);
        Self { vals: h, conf: v }
    }
    fn set_value(hash: &mut HashMap<String, Value>, key: String, value: String) {
        match hash.get_mut(&key).expect("Key not valid") {
            Value::Simple(s, _o) => *s = value,
            Value::Selected(s, l, _o) => {
                if l.contains(&value) {
                    *s = value
                } else {
                    panic!("{} is not valid for {}", value, key)
                }
            }
            Value::Range(i, r, _o) => {
                if r.contains(&value.parse().expect("Expected Number")) {
                    *i = value.parse().unwrap()
                } else {
                    panic!("{} is not valid for {}", value, key)
                }
            }
            Value::IP(ip, _o) => *ip = IPv4::parse(&value),
            Value::Interface(it, _o) => *it = Iface::parse(&value),
            _ => (),
        }
    }
    pub fn set(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        Self::set_value(&mut self.vals, key.into(), value.into());
        self
    }
    pub fn set_present(mut self, key: impl Into<String>) -> Self {
        match self.vals.get_mut(&key.into()).expect("Key not valid") {
            Value::Selected(s, l, _b) => {
                if l.len() == 1 {
                    *s = l[0].to_string()
                } else {
                    panic!("Key has too many options")
                }
            }
            _ => panic!("Key doesn't have options"),
        }
        self
    }
    pub fn add_list(mut self, key: impl Into<String>) -> Self {
        match self.vals.get_mut(&key.into()).expect("Key doesn't exist") {
            Value::List(v) => {
                v.push(v[0].clone());
            }
            _ => panic!("Key isn't a list"),
        }
        self
    }
    pub fn add_list_count(mut self, key: impl Into<String>, num: usize) -> Self {
        let key_s = key.into();
        for _ in 0..num {
            match self.vals.get_mut(&key_s).expect("Key doesn't exist") {
                Value::List(v) => {
                    v.push(v[0].clone());
                }
                _ => panic!("Key isn't a list"),
            }
        }
        self
    }
    pub fn set_list_item(
        mut self,
        keys: &[(&str, usize)],
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        let mut cur = &mut self.vals;
        for (k, num) in keys {
            match cur.get_mut(&k.to_string()).expect("") {
                Value::List(v) => {
                    cur = &mut v[*num];
                }
                _ => panic!(""),
            }
        }
        Self::set_value(cur, key.into(), value.into());
        self
    }
    pub fn add_list_vec(mut self, key: impl Into<String>, vals: Vec<(&str, &str)>) -> Self {
        match self.vals.get_mut(&key.into()).expect("Key doesn't exist") {
            Value::List(v) => {
                let mut new = v[0].clone();
                for (k, v) in vals {
                    Self::set_value(&mut new, k.to_owned(), v.to_owned());
                }
                v.push(new);
            }
            _ => panic!("Key isn't a list"),
        }
        self
    }
    fn compile_line(mut tmp: &str, vals: &HashMap<String, Value>) -> String {
        let mut ret = String::default();
        // `?\` is an optional command, ignored for now
        let opt = if tmp.starts_with("?\\") {
            tmp = &tmp[2..];
            true
        } else {
            false
        };
        for w in tmp.split_whitespace().filter(|&w| w != "") {
            if w.starts_with('"') {
                ret += "\"";
            }
            let word = w.trim_matches('"');
            if word.starts_with("$") || word.starts_with("?") {
                let mut parts = word
                    .split(|c| c == '$' || c == '?' || c == '{' || c == '}')
                    .filter(|&s| s != "");
                let mut name_parts = parts.next().unwrap().split("-").filter(|&s| s != "");
                let a = vals.get(name_parts.next().unwrap());
                if a.is_none() {
                    panic!("Failed at {}, vals: {:?}", word, vals);
                }
                match a.unwrap() {
                    Value::Simple(s, b) => {
                        if s == "" && !b {
                            if opt {
                                return String::default();
                            }
                            panic!("Required value not supplied for {}", word);
                        } else if s != "" {
                            ret += &s;
                            if w.ends_with('"') {
                                ret += "\"";
                            }
                            ret += " ";
                        }
                    }
                    Value::Selected(s, _l, b) => {
                        if s == "" && !b {
                            if opt {
                                return String::default();
                            }
                            panic!("Required value not supplied for {}", word);
                        } else if s != "" {
                            ret += &s;
                            if w.ends_with('"') {
                                ret += "\"";
                            }
                            ret += " ";
                        }
                    }
                    Value::Range(i, _r, b) => {
                        if *i == -1 && !b {
                            if opt {
                                return String::default();
                            }
                            panic!("Required value not supplied for {}", word);
                        } else {
                            if *i != -1 {
                                ret += &i.to_string();
                                if w.ends_with('"') {
                                    ret += "\"";
                                }
                                ret += " ";
                            }
                        }
                    }
                    Value::IP(ip, b) => {
                        if ip.ip_type() == IPType::None && !b {
                            if opt {
                                return String::default();
                            }
                            panic!("Required value not supplied for {}", word);
                        } else if ip.ip_type() != IPType::None {
                            if let Some(t) = name_parts.next() {
                                if t == "mask" {
                                    ret += &format!("{:#}", ip.subnet_mask());
                                } else {
                                    panic!("Malformed name {}", word);
                                }
                            } else {
                                if let Some(type_name) = parts.next() {
                                    if type_name == "\\ipslash" {
                                        ret += &format!("{}", ip);
                                    } else {
                                        ret += &format!("{:#}", ip);
                                    }
                                } else {
                                    ret += &format!("{:#}", ip);
                                }
                            }
                            if w.ends_with('"') {
                                ret += "\"";
                            }
                            ret += " ";
                        }
                    }
                    Value::Interface(it, b) => {
                        if it.is_none() && !b {
                            if opt {
                                return String::default();
                            }
                            panic!("Required value not supplied for {}", word);
                        } else if !it.is_none() {
                            ret += &it.fmt();
                            if w.ends_with('"') {
                                ret += "\"";
                            }
                            ret += " ";
                        }
                    }
                    Value::List(_l) => unreachable!(),
                }
            } else if word.starts_with("\\$") {
                ret += &word[1..];
                if w.ends_with('"') {
                    ret += "\"";
                }
                ret += " ";
            } else {
                ret += word;
                if w.ends_with('"') {
                    ret += "\"";
                }
                ret += " ";
            }
        }
        ret
    }
    fn compile_pattern(
        v: &Vec<String>,
        i: &mut usize,
        vals: Option<&HashMap<String, Value>>,
    ) -> Vec<String> {
        // let mut i = v.iter();
        let mut ret = Vec::new();
        while *i < v.len() {
            let tmp: &str = v[*i].trim();
            // `!` is ignored, excpet when parsing for loops
            if tmp.starts_with("!") || tmp.starts_with("#") {
                if tmp.contains("! for $") || tmp.contains("# for $") {
                    let name = tmp[7..].split_whitespace().next().expect("").to_string();
                    if let Some(val) = vals {
                        match val.get(&name).expect(&name) {
                            Value::List(list) => {
                                *i += 1;
                                for map in list.iter().skip(1) {
                                    let mut start = *i;
                                    ret.append(&mut Self::compile_pattern(
                                        &v,
                                        &mut start,
                                        Some(map),
                                    ));
                                }
                                Self::compile_pattern(&v, i, None);
                            }
                            _ => panic!("bad"),
                        }
                    } else {
                        Self::compile_pattern(&v, i, None);
                    }
                } else if tmp.contains("! end for")
                    || tmp.contains("! end conf")
                    || tmp.contains("# end for")
                    || tmp.contains("# end conf")
                {
                    *i += 1;
                    break;
                } else {
                    *i += 1;
                }
            } else {
                if let Some(val) = vals {
                    ret.push(Self::compile_line(tmp, &val));
                }
                *i += 1;
            }
        }
        ret
    }
    pub fn compile(self) -> Vec<String> {
        Self::compile_pattern(&self.conf, &mut 0, Some(&self.vals))
    }
    #[allow(unused)]
    pub fn debug(self) -> Self {
        println!("{:?}", self);
        self
    }
}

impl std::fmt::Debug for Conf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Conf {:?}", self.vals)
    }
}
