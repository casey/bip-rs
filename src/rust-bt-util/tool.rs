extern crate serialize;
extern crate "rust-bt" as rust_bt;
extern crate "rust-crypto" as crypto;

fn main() {
    use std::io::{IoResult};
    use std::io::net::ip::{SocketAddr, Ipv4Addr, Ipv6Addr, IpAddr};
    use std::io::net::udp::{UdpSocket};
    use std::io::fs::File;
    use std::io::net::addrinfo::get_host_addresses;
    use std::u16;

    use serialize::hex::ToHex;
    use crypto::sha1::Sha1;
    use crypto::digest::Digest;
    use rust_bt::bencode::BenValue;
    use rust_bt::tracker_udp::UdpTracker;
    use rust_bt::tracker::Tracker;
    use rust_bt::torrent::{Torrent};
    use rust_bt::upnp::UPnPInterface;
    use rust_bt::upnp::ServiceDesc;
    
    let mut torr_file = File::open(&Path::new("tests/data/udp_tracker/sample.torrent"));
    let torr_bytes = match torr_file.read_to_end() {
        Ok(n)  => n,
        Err(n) => { println!("{}", n); return }
    };
    
    let ben_val: BenValue = match BenValue::new(torr_bytes.as_slice()) {
        Ok(n) => n,
        Err(n) => { println!("{}", n); return }
    };
    let torrent = Torrent::new(&ben_val);
    
    if torrent.is_err() {
        println!("{}", torrent.err().unwrap());
        return;
    }
    let torrent = torrent.unwrap();
    
    let dict = ben_val.dict().expect("1");
    
    let announce_url = dict.find_equiv(&"announce").expect("2").str().expect("3");
    
    let mut sha = Sha1::new();
    let mut result = [0u8,..20];
    let encoded = dict.find_equiv(&"info").expect("4").encoded();
    
    sha.input(encoded.as_slice());
    sha.result(result);

    println!("{}", torrent.announce);
    
    //println!("{}", result.to_hex());
    
    match UPnPInterface::find_services(SocketAddr{ ip: Ipv4Addr(192, 168, 1, 102), port: 1901 }, "WANIPConnection", "1") {
        Ok(n) => {
            for i in n.iter() {
                let service_desc = i.service_desc().unwrap();
                service_desc.soap_request("GetExternalIPAddress", []).unwrap();
            }
        },
        Err(n) => println!("{} saqS", n)
    };
}