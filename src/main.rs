mod ip;
mod config;
mod switch;
use ip::IP;

fn main() {
    let ip = ip::IPv4::parse("195.1.1.0/24");
    let nets = ip::IPv4::build_net(ip.network(), 3);
    println!("  Address  \t    Broadcast\t");
    for n in nets {
        println!("{}\t| {}\t", n, n.broadcast());
    }
}
