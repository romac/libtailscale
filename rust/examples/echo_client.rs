use std::{env, io::Write};

use tsnet::{Network, ServerBuilder};

fn main() {
    let target = env::args().nth(1).expect("usage: echoclient host:port");

    let srv = ServerBuilder::new()
        .hostname("libtailscale-rs-echoclient")
        .ephemeral()
        .authkey(env::var("TS_AUTHKEY").expect("set TS_AUTHKEY in environment"))
        .build()
        .unwrap();

    let mut conn = srv.connect(Network::Tcp, &target).unwrap();
    writeln!(conn, "This is a test of the Tailscale connection service.").unwrap();
}
