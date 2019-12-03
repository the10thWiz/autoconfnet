use std::ops::Range;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum InterfaceType {
    GigabitEthernet,
    FastEthernet,
    Ethernet,
    Serial,
    Loopback,
}

#[derive(Debug, Clone)]
pub struct Interface {
    itype: InterfaceType,
    num: (u8, u8, u8),
    num_s: String,
}

#[derive(Debug, Clone)]
pub enum Iface {
    Range(Range<Interface>),
    Single(Interface),
    None,
}

impl Iface {
    pub fn parse(s: &str) -> Self {
        let s = s.trim().to_ascii_lowercase();
        let itype = if s.starts_with("g") {
            InterfaceType::GigabitEthernet
        } else if s.starts_with("f") {
            InterfaceType::FastEthernet
        } else if s.starts_with("e") {
            InterfaceType::Ethernet
        } else if s.starts_with("s") {
            InterfaceType::Serial
        } else if s.starts_with("l") {
            InterfaceType::Loopback
        } else {
            panic!("Malformed Interface type")
        };
        let mut num = ((-1i8, -1i8), (-1i8, -1i8), (-1i8, -1i8));
        let mut nums = s
            .split_whitespace()
            .flat_map(|sp| sp.chars())
            .skip_while(|c| c.is_alphabetic());
        let a = nums.by_ref().take_while(|&c| c != '/').collect::<String>();
        let mut a = a.split("-");
        num.0 .0 = a
            .next()
            .expect("Doesn't have a number")
            .parse()
            .expect("Wasn't a number");
        if let Some(a) = a.next() {
            num.0 .1 = a.parse().expect("Wasn't a number");
        } else {
            num.0 .1 = num.0 .0;
        }
        let a = nums.by_ref().take_while(|&c| c != '/').collect::<String>();
        let mut a = a.split("-").filter(|&s| s != "");
        if let Some(n) = a.next() {
            num.1 .0 = n.parse().expect("Wasn't a number");
            if let Some(a) = a.next() {
                num.1 .1 = a.parse().expect("Wasn't a number");
            } else {
                num.1 .1 = num.1 .0;
            }
            let a = nums.by_ref().take_while(|&c| c != '/').collect::<String>();
            let mut a = a.split("-").filter(|&s| s != "");
            if let Some(n) = a.next() {
                num.1 .0 = n.parse().expect("Wasn't a number");
                if let Some(a) = a.next() {
                    num.1 .1 = a.parse().expect("Wasn't a number");
                } else {
                    num.2 .1 = num.2 .0;
                }
            }
        }
        if num.0 .1 == -1 && num.1 .1 == -1 && num.2 .1 == -1 {
            Iface::Single(Interface::new(
                itype,
                (num.0 .0 as u8, num.1 .0 as u8, num.2 .0 as u8),
            ))
        } else {
            Iface::Range(
                Interface::new(itype, (num.0 .0 as u8, num.1 .0 as u8, num.2 .0 as u8))
                    ..Interface::new(itype, (num.0 .1 as u8, num.1 .1 as u8, num.2 .1 as u8)),
            )
        }
    }
    pub fn fmt(&self) -> String {
        match self {
            Self::None => format!(""),
            Self::Single(i) => format!("{} {}", i.fmt_name(), i.fmt_num()),
            Self::Range(r) => format!(
                "range {} {}",
                r.start.fmt_name(),
                r.start.fmt_num_range(&r.end)
            ),
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }
}

impl Default for Iface {
    fn default() -> Self {
        Self::None
    }
}

impl std::ops::Index<String> for Interface {
    type Output = String;

    fn index(&self, i: String) -> &Self::Output {
        match &i[..] {
            "num" => &self.num_s,
            _ => panic!("{} is not a valid interface part", i),
        }
    }
}

impl Interface {
    fn new(t: InterfaceType, num: (u8, u8, u8)) -> Interface {
        Interface {
            num_s: match t {
                InterfaceType::Serial => format!("{}/{}/{}", num.0, num.1, num.2),
                InterfaceType::Loopback => format!("{}", num.0),
                InterfaceType::Ethernet
                | InterfaceType::FastEthernet
                | InterfaceType::GigabitEthernet => format!("{}/{}", num.0, num.1),
            },
            num,
            itype: t,
        }
    }
    fn fmt_name(&self) -> String {
        match self.itype {
            InterfaceType::Ethernet => "Ethernet".to_string(),
            InterfaceType::FastEthernet => "FastEthernet".to_string(),
            InterfaceType::GigabitEthernet => "GigabitEthernet".to_string(),
            InterfaceType::Loopback => "Looback".to_string(),
            InterfaceType::Serial => "Serial".to_string(),
        }
    }
    fn fmt_num(&self) -> String {
        match self.itype {
            InterfaceType::Serial => format!("{}/{}/{}", self.num.0, self.num.1, self.num.2),
            InterfaceType::Loopback => format!("{}", self.num.0),
            InterfaceType::Ethernet
            | InterfaceType::FastEthernet
            | InterfaceType::GigabitEthernet => format!("{}/{}", self.num.0, self.num.1),
        }
    }
    fn fmt_num_range(&self, other: &Self) -> String {
        match self.itype {
            InterfaceType::Serial => format!(
                "{}/{}/{}",
                Self::fmt_range(self.num.0, other.num.0),
                Self::fmt_range(self.num.1, other.num.1),
                Self::fmt_range(self.num.2, other.num.2)
            ),
            InterfaceType::Loopback => format!("{}", Self::fmt_range(self.num.0, other.num.0)),
            InterfaceType::Ethernet
            | InterfaceType::FastEthernet
            | InterfaceType::GigabitEthernet => format!(
                "{}/{}",
                Self::fmt_range(self.num.0, other.num.0),
                Self::fmt_range(self.num.1, other.num.1)
            ),
        }
    }
    fn fmt_range(a: u8, b: u8) -> String {
        if a == b {
            format!("{}", a)
        } else {
            format!("{} - {}", a, b)
        }
    }
}

fn split(s: &str) -> Vec<String> {
    let mut ret = Vec::new();
    let mut cur = String::default();
    let mut t = 0;
    for c in s.chars() {
        match t {
            1 => {
                if c.is_alphabetic() {
                    cur.push(c);
                    break;
                }
            }
            2 => {
                if c.is_numeric() {
                    cur.push(c);
                    break;
                }
            }
            _ => (),
        }
        if c.is_alphabetic() {
            if cur != "" {
                ret.push(cur);
            }
            cur = String::default();
            cur.push(c);
            t = 1;
        } else if c.is_numeric() {
            if cur != "" {
                ret.push(cur);
            }
            cur = String::default();
            cur.push(c);
            t = 2;
        } else if c.is_numeric() {
            if cur != "" {
                ret.push(cur);
            }
            cur = String::default();
            cur.push(c);
            t = 0;
        }
    }
    ret
}
