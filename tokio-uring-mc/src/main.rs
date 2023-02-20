use clap::Parser;
use socket2::{Protocol, Socket, Type};
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio_uring::net::UdpSocket;

pub const MAX_DATAGRAM_SIZE: usize = 65507;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, disable_colored_help = true)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    server: bool,

    #[arg(short, long, default_value_t = Ipv4Addr::UNSPECIFIED)]
    bind_addr: Ipv4Addr,

    #[arg(short, long)]
    mc_addr: Ipv4Addr,

    #[arg(short, long, default_value_t = 128)]
    data_size: usize,

    #[arg(short, long, default_value_t = 10)]
    count: usize,
}

fn run_server(args: Args) -> std::io::Result<()> {
    tokio_uring::start(async {
        let std_addr: SocketAddrV4 = format!("{}:0", args.bind_addr).parse().unwrap();
        let mc_addr: SocketAddrV4 = format!("{}:50001", args.mc_addr).parse().unwrap();
        let sock = Socket::new(socket2::Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

        sock.set_reuse_port(true)?;
        sock.set_nonblocking(true)?;
        sock.bind(&std_addr.into())?;

        let std_socket = UdpSocket::from_std(sock.into());

        // write data
        for i in 1..=args.count {
            let mut data = i.to_string().as_bytes().to_owned();
            data.resize(args.data_size, 0_u8);
            let (result, _) = std_socket.send_to(data, mc_addr.into()).await;
            result?;
        }
        Ok(())
    })
}

fn run_client(args: Args) -> std::io::Result<()> {
    tokio_uring::start(async {
        let mc_addr: SocketAddrV4 = format!("{}:50001", args.mc_addr).parse().unwrap();
        let sock = Socket::new(socket2::Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

        sock.set_reuse_port(true)?;
        sock.set_nonblocking(true)?;
        sock.bind(&mc_addr.into())?;
        sock.join_multicast_v4(mc_addr.ip(), &Ipv4Addr::UNSPECIFIED)?;

        let std_socket = UdpSocket::from_std(sock.into());

        // read data
        for i in 1..=args.count {
            let buf = vec![0; MAX_DATAGRAM_SIZE];

            let (result, buf) = std_socket.recv_from(buf).await;
            let (n_bytes, _) = result?;
            let mut data = i.to_string().as_bytes().to_owned();
            data.resize(args.data_size, 0_u8);

            assert_eq!(data, &buf[..n_bytes]);
        }
        Ok(())
    })
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    match args.server {
        true => run_server(args),
        false => run_client(args),
    }
}
