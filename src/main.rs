mod config;
mod file;
mod interface;
mod ip;
mod output;
mod parse;
mod switch;
use output::WriteProc;

fn main() {
    parse::Conf::parse("router.conf")
        .set("password", "cisco")
        .set("secret", "class")
        .set("hostname", "Gateway")
        .set("banner", "Authorized Access Only")
        .set_present("nodns")
        .add_list_vec(
            "interface",
            vec![
                ("iface", "f0/1"),
                ("ip", "192.168.1.1/24"),
                ("description", "Main fast ethernet"),
            ],
        )
        .add_list_vec(
            "interface",
            vec![
                ("iface", "s0/0/1"),
                ("ip", "209.165.201.18/30"),
                ("description", "Serial to ISP"),
            ],
        )
        .compile().write_to_clip();
        // .iter(); //.for_each(|l| println!("{}", l));

    // parse::Conf::parse("switch.conf")
    //     .set("password", "cisco")
    //     .set("secret", "class")
    //     .set("hostname", "S1")
    //     .set("banner", "Authorized Access Only")
    //     .set("ssh.timeout", "10")
    //     .set("ssh.retries", "3")
    //     .set("domain", "fake")
    //     .set("username", "fake")
    //     .set("level", "1")
    //     .set("user.password", "fake")
    //     .set("svi.ip", "192.168.1.11/24")
    //     .set("gateway", "192.168.1.1/24")
    //     .set_present("nodns")
    //     .add_list_count("unused", 2)
    //     .set_list(&[("unused", 1)], "iface", "f0/2-4")
    //     .set_list(&[("unused", 2)], "iface", "f0/7-24")
    //     .compile();//.iter().for_each(|l| println!("{}", l));
    // parse::Conf::parse("local.conf")
    //     .set("ip", "192.168.10.3/24")
    //     .set("device", "eno1")
    //     .add_list_count("route", 1)
    //     .set_list_item(&[("route", 1)], "ip", "192.168.10.0/24")
    //     .set_list_item(&[("route", 1)], "device", "eno1")
    //     .compile().write_to_clip();
}
