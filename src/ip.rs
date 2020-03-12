use std::fmt;
use std::default::Default;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IPType {
    Network,
    SubnetMask,
    SubnetAddr,
    Public,
    Broadcast,
    DHCP,
    Empty,
    None,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IPClass {
    A,
    B,
    C,
    Unknown,
}

pub enum IPError {
    BadMask(u32),
}

pub trait IP: fmt::Display + Copy + Eq {
    fn next(&self) -> Option<Self>;
    fn first(&self) -> Self;
    fn last(&self) -> Self;
    fn ip_type(&self) -> IPType;
    fn network(&self) -> Self;
    fn subnet_mask(&self) -> Self;
    fn subnet_addr(&self) -> Self;
    fn subnet_net(&self) -> Self;
    fn subnet_net_num(&self) -> Self;
    fn broadcast(&self) -> Self;
    fn class(&self) -> IPClass;
    fn is_valid(&self) -> Result<(), IPError>;
    fn num_hosts(&self) -> u32;
}

const IP_4_PART: u32 = 0xFF; // 8 bits
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IPv4 {
    ip: u32,
    mask: u32,
    super_mask: u32,
    ip_type: IPType,
}

impl IPv4 {
    fn ip_part(s: Option<&str>, part: u32) -> u32 {
        let ret = s
            .expect("Expected at least 4 nums")
            .parse::<u32>()
            .expect("IP parts are ints");
        if ret > 0xFF {
            panic!("{} is not a valid ip part", ret);
        }
        ret << part
    }
    pub fn parse(ip: &str) -> Self {
        if ip == "dhcp" {
            Self {
                ip: 0,
                mask: 0,
                super_mask: 0,
                ip_type: IPType::DHCP,
            }
        } else {
            let mut tmp = ip.split('/');
            // ip addr
            let mut ip_split = tmp.next().expect("Address expected").split('.');
            let ip = Self::ip_part(ip_split.next(), 24)
                | Self::ip_part(ip_split.next(), 16)
                | Self::ip_part(ip_split.next(), 8)
                | Self::ip_part(ip_split.next(), 0);
            if !ip_split.next().is_none() {
                panic!("Too many `.` in ip: {}", ip);
            }
            // mask
            let mask = u32::max_value()
                << (32
                    - tmp
                        .next()
                        .expect("Mask not provided")
                        .parse::<u32>()
                        .expect("Mask was not int"));
            // check format
            if !tmp.next().is_none() {
                panic!("Too many `/` in ip: {}", ip);
            }
            if ip & mask == 0 {
                Self {
                    ip,
                    mask,
                    super_mask: 0,
                    ip_type: IPType::Network,
                }
            } else {
                Self {
                    ip,
                    mask,
                    super_mask: 0,
                    ip_type: IPType::Public,
                }
            }
        }
    }
    fn mask_num(&self) -> u32 {
        let mut ret = 0;
        for i in 0..32 {
            if self.mask & (1 << i) != 0 {
                ret += 1;
            }
        }
        ret
    }
    pub fn build_net(network: Self, num_net: u32) -> Vec<Self> {
        // num_net <= 2^n
        let mut bits = 0;
        for i in 0..32 - network.mask_num() {
            if num_net <= (1 << i) {
                bits = i;
                break;
            }
        }
        // sub sub net bits = bits
        let mask = u32::max_value() << (32 - (network.mask_num() + bits));
        let mut ret = Vec::new();
        for i in 0..1 << bits {
            ret.push(Self {
                ip: network.ip | (i << (32 - (network.mask_num() + bits))),
                mask,
                super_mask: network.mask,
                ip_type: IPType::Network,
            });
        }
        ret
    }
}

impl Default for IPv4 {
    fn default() -> Self {
        Self {
            ip: 0,
            mask: 0,
            super_mask: 0,
            ip_type: IPType::None,
        }
    }
}

impl IP for IPv4 {
    fn next(&self) -> Option<Self> {
        if self.ip & self.mask != (self.ip + 1) & self.mask {
            None
        } else {
            Some(Self {
                ip: self.ip + 1,
                mask: self.mask,
                super_mask: self.super_mask,
                ip_type: self.ip_type,
            })
        }
    }
    fn first(&self) -> Self {
        Self {
            ip: (self.ip & self.mask) + 1,
            mask: self.mask,
            super_mask: self.super_mask,
            ip_type: self.ip_type,
        }
    }
    fn last(&self) -> Self {
        Self {
            ip: (self.ip & self.mask) + (u32::max_value() & !self.mask) - 1,
            mask: self.mask,
            super_mask: self.super_mask,
            ip_type: self.ip_type,
        }
    }
    fn ip_type(&self) -> IPType {
        self.ip_type
    }
    fn network(&self) -> Self {
        Self {
            ip: self.ip & (self.mask),
            mask: self.mask,
            super_mask: self.super_mask,
            ip_type: IPType::Network,
        }
    }
    fn subnet_mask(&self) -> Self {
        if self.ip_type == IPType::DHCP {
            Self {
                ip: 0,
                mask: 0,
                super_mask: 0,
                ip_type: IPType::Empty,
            }
        }else {
            Self {
                ip: self.mask,
                mask: 0,
                super_mask: 0,
                ip_type: IPType::SubnetMask,
            }
        }
    }
    fn subnet_addr(&self) -> Self {
        Self {
            ip: self.ip & (!self.mask),
            mask: self.mask,
            super_mask: self.super_mask,
            ip_type: IPType::SubnetAddr,
        }
    }
    fn subnet_net(&self) -> Self {
        Self {
            ip: self.ip & (self.mask ^ self.super_mask),
            mask: self.mask,
            super_mask: self.super_mask,
            ip_type: IPType::SubnetAddr,
        }
    }
    fn subnet_net_num(&self) -> Self {
        Self {
            ip: (self.ip & (self.mask ^ self.super_mask)) >> (32 - self.mask_num()),
            mask: self.mask,
            super_mask: self.super_mask,
            ip_type: IPType::SubnetAddr,
        }
    }
    fn broadcast(&self) -> Self {
        Self {
            ip: (self.ip & self.mask) + (u32::max_value() & !self.mask),
            mask: self.mask,
            super_mask: self.super_mask,
            ip_type: IPType::Broadcast,
        }
    }
    fn class(&self) -> IPClass {
        match (self.ip >> 24) & IP_4_PART {
            0..=127 => IPClass::A,
            128..=191 => IPClass::B,
            192..=223 => IPClass::C,
            _ => IPClass::Unknown,
        }
    }
    fn is_valid(&self) -> Result<(), IPError> {
        let mut end = false;
        for i in (0..32).rev() {
            if self.mask & (1 << i) == 0 {
                end = true;
            } else if end {
                return Err(IPError::BadMask(self.mask));
            }
        }
        if self.mask & 1 == 1 {
            return Err(IPError::BadMask(self.mask));
        }
        Ok(())
    }
    fn num_hosts(&self) -> u32 {
        self.last().ip - self.first().ip
    }
}

impl fmt::Display for IPv4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.ip_type == IPType::DHCP {
            write!(f, "dhcp")
        } else if self.ip_type == IPType::Empty {
            Ok(())
        } else {
            if f.alternate() || self.ip_type == IPType::SubnetMask {
                write!(
                    f,
                    "{}.{}.{}.{}",
                    (self.ip >> 24) & IP_4_PART,
                    (self.ip >> 16) & IP_4_PART,
                    (self.ip >> 8) & IP_4_PART,
                    (self.ip) & IP_4_PART
                )
            } else {
                write!(
                    f,
                    "{}.{}.{}.{}/{}",
                    (self.ip >> 24) & IP_4_PART,
                    (self.ip >> 16) & IP_4_PART,
                    (self.ip >> 8) & IP_4_PART,
                    (self.ip) & IP_4_PART,
                    self.mask_num()
                )
            }
        }
    }
}

impl fmt::Binary for IPv4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() || self.ip_type == IPType::SubnetMask {
            write!(
                f,
                "{:08b}.{:08b}.{:08b}.{:08b}",
                (self.ip >> 24) & IP_4_PART,
                (self.ip >> 16) & IP_4_PART,
                (self.ip >> 8) & IP_4_PART,
                (self.ip) & IP_4_PART
            )
        } else {
            write!(
                f,
                "{:08b}.{:08b}.{:08b}.{:08b}/{}",
                (self.ip >> 24) & IP_4_PART,
                (self.ip >> 16) & IP_4_PART,
                (self.ip >> 8) & IP_4_PART,
                (self.ip) & IP_4_PART,
                self.mask_num()
            )
        }
    }
}
