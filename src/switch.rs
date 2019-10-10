
use crate::config::Config;
use crate::ip::IP;

enum Service {
    Timestamps(bool, TimestampType),// (no)? service timestamps
    Encryption(bool),// (no)? service password-encryption
    DHCP(bool),// (no)? service dhcp
}
enum TimestampType {
    Debug,// debug datetime msec
    Log,// log datetime msec
}
impl Service {
    fn to_string(&self) -> String {
        let mut ret = String::default();
        match *self {
            Service::DHCP(b) | Service::Encryption(b) | Service::Timestamps(b, _) => if b {ret.push_str("no ")},
        }
        ret.push_str("service ");
        match self {
            Service::DHCP(_) => ret.push_str("dhcp"),
            Service::Encryption(_) => ret.push_str("password-encryption"),
            Service::Timestamps(_, t) => {
                ret.push_str("timestamps ");
                match t {
                    TimestampType::Debug => ret.push_str("debug datetime msec"),
                    TimestampType::Log => ret.push_str("log datetime msec"),
                }
            },
        }
        ret.push_str("\n");
        ret
    }
}
struct VlanRange {
    start: usize,
    end: usize,
}
impl VlanRange {
    fn to_string(&self) -> String {
        let mut ret = String::default();
        ret+= &self.start.to_string();
        if self.end != 0 {
            ret+= "-";
            ret+= &self.end.to_string();
        }
        ret
    }
}
enum SpanningTree {
    Mode(SpanningTreeMode),
    Portfast,
    Vlan(VlanRange),
}
enum SpanningTreeMode {
    Pvst,
    RapidPvst,
}
impl SpanningTree {
    fn to_string(&self) -> String {
        let mut ret = String::default();
        ret+= "spanning-tree ";
        match self {
            Self::Mode(s) => {
                ret+= "mode ";
                match s {
                    SpanningTreeMode::Pvst => ret+= "pvst",
                    SpanningTreeMode::RapidPvst => ret+= "rapid-pvst",
                }
            },
            Self::Portfast => ret+= "portfast default",
            Self::Vlan(v) => {
                ret+= "vlan ";
                ret+= &v.to_string();
            },
        }
        ret
    }
}
#[derive(PartialEq, Eq)]
enum InterfaceType {
    Ethernet(usize, usize),
    FastEthernet(usize, usize),
    GigabitEthernet(usize, usize),
    PortChannel(usize),
    Vlan(usize),
    None,
}
impl InterfaceType {
    fn to_string(&self) -> String {
        let mut ret = String::default();
        match self {
            Self::Ethernet(a, b) => {
                ret+= "Ethernet ";
                ret+= &a.to_string();
                ret+= "/";
                ret+= &b.to_string();
            },
            Self::FastEthernet(a, b) => {
                ret+= "FastEthernet ";
                ret+= &a.to_string();
                ret+= "/";
                ret+= &b.to_string();
            },
            Self::GigabitEthernet(a, b) => {
                ret+= "GigabitEthernet ";
                ret+= &a.to_string();
                ret+= "/";
                ret+= &b.to_string();
            },
            Self::PortChannel(a) => {
                ret+= "Port-channel ";
                ret+= &a.to_string();
            },
            Self::Vlan(a) => {
                ret+= "Vlan ";
                ret+= &a.to_string();
            },
            Self::None => (),
        }
        ret
    }
}
enum GroupAction{

}
enum ChannelProtocol {
    Lacp,
    Pagp,
}
enum DHCPSnoop {
    Limit(usize),
    Trust,
}
enum MLSAction {
    Cos(usize),
    Trust,
}
enum SpanningTreeI {

}
enum Speed {
    S10,
    S100,
    Auto,
}
enum SwitchPort {

}
struct Interface {
    name: (InterfaceType, InterfaceType, bool),
    cpd: bool,
    channel_group: (usize, GroupAction),
    channel_protocol: (ChannelProtocol),
    description: String,
    duplex: Option<bool>,
    ip: Vec<DHCPSnoop>,
    lldp: (bool, bool),
    mdix: bool,
    mls: Vec<MLSAction>,
    shutdown: bool,
    spanning_tree: Vec<SpanningTreeI>,
    speed: Speed,
    storm_control: f64,
    switchport: Vec<SwitchPort>,
    tx_ring_limit: usize,
}
impl Interface {
    fn to_string(&self) -> String {
        let mut ret = String::default();
        ret+= "interface ";
        ret+= &self.name.0.to_string();
        if self.name.1 != InterfaceType::None {
            ret+= if self.name.2 {" , "} else {" - "};
            ret+= &self.name.1.to_string();
        }
        ret+= "\n";
        
        ret+= "description \"";
        ret+= &self.description;
        ret+= "\"";

        ret+= if self.cpd {""} else {"no "};
        ret+= "cpd enable\n";


        
        ret+= "exit\n";
        ret
    }
}
enum Line {

}
struct Access {

}
enum Timezone {

}
enum KeyAction {

}
enum IPAction {

}
struct LogAction {

}
struct StaticMacAddress {

}
struct MonitorSession {

}
struct NTPAction {

}
struct Vlan {

}
struct VTPAction {

}

struct SwitchConfig {
    // version 12.2
    services: Vec<Service>,// service
    hostname: String,// hostname <str>
    banner: String,// banner <str>
    clock: Timezone,// clock timezone
    spanning_tree: Vec<SpanningTree>,// spanning-tree
    access_list: Vec<Access>,// access-list
    crypto: Vec<KeyAction>,// crypto key
    cdp: bool,// (no)? cdp run
    lldp: bool,// (no)? lldp run
    mls: bool,// (no)? mls qos
    interface: Vec<Interface>,// interface
    vlan: Vec<Vlan>,// vlan
    logging: Vec<LogAction>,// logging
    mac_table: Vec<StaticMacAddress>,// mac-address-table static
    monitor: Vec<MonitorSession>,// monitor session
    ntp: Vec<NTPAction>,// ntp
    ip: Vec<IPAction>,// ip
    line: Vec<Line>,// line
    vtp: Vec<VTPAction>,// vtp
    // end
}
const EMPTY_LINE : &str = "!\n";
impl SwitchConfig {
    fn hostname(&self) -> String {
        let mut ret = String::default();
        ret+= "hostname \"";
        ret+= &self.hostname;
        ret+= "\"\n";
        ret
    }
    fn banner(&self) -> String {
        let mut ret = String::default();
        ret+= "banner \"";
        ret+= &self.banner;
        ret+= "\"\n";
        ret
    }
}

impl Config for SwitchConfig {
    fn make_string(&self) -> String {
        let mut ret = String::default();
        ret.push_str(EMPTY_LINE);
        self.services.iter().for_each(|s|ret+= &s.to_string());
        ret.push_str(EMPTY_LINE);
        ret+= &self.hostname();
        ret.push_str(EMPTY_LINE);
        ret+= &self.banner();
        ret.push_str(EMPTY_LINE);
        self.spanning_tree.iter().for_each(|s|ret+= &s.to_string());
        ret.push_str(EMPTY_LINE);
        ret
    }
}
