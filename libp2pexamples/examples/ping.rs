use async_std::task;
use futures::{future, prelude::*};
use libp2p::{identity, PeerId, ping::{Ping, PingConfig}, Swarm};
use std::{error::Error, task::{Context, Poll}};
use env_logger;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    //create a random peer id
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {:?}", peer_id);

    //create a transport
    let transport = libp2p::build_development_transport(id_keys)?;

    //create a ping network behavior
    let behavior = Ping::new(PingConfig::new().with_keep_alive(true));
    
    let mut swarm = Swarm::new(transport, behavior, peer_id);

    if let Some(addr) = std::env::args().nth(1) {
        let remote = addr.parse()?;
        Swarm::dial_addr(&mut swarm, remote)?;
        println!("dialled {}", addr)
    }

    Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse()?)?;

    let mut listening = false;

    task::block_on(future::poll_fn(move |cx: &mut Context<'_>| {
        loop {
            match swarm.poll_next_unpin(cx) {
                Poll::Ready(Some(event)) => println!("{:?}", event),
                Poll::Ready(None) => return Poll::Ready(()),
                Poll::Pending => {
                    if !listening {
                        for  addr in Swarm::listeners(&swarm) {
                            println!("Listening on {}", addr);
                            listening = true;
                        }
                    }
                    return Poll::Pending;
                } 
            }
        }
    }));
    Ok(())  
}
