use std::{
    io::Error,
    io::Read,
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
};

pub fn viv_listener() -> Result<(), Error> {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 0);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;
    println!(
        "\u{26EC} Listenning on {}, access this port to end the program",
        port
    );

    let (mut tcp_stream, addr) = listener.accept()?; // block until requested
    println!("\u{26EC} Connection received! {:?} is sending data.", addr);
    let mut input = String::new();
    let _ = tcp_stream.read_to_string(&mut input)?;
    println!("\u{26EC} {:?} says {}", addr, input);
    Ok(())
}
