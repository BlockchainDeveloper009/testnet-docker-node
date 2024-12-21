// use libp2p::{
//     core::upgrade,
//     floodsub::{Floodsub, FloodsubEvent, Topic},
//     mdns::{Mdns, MdnsEvent},
//     swarm::{NetworkBehaviourEventProcess, Swarm, SwarmBuilder},
//     NetworkBehaviour, PeerId, Transport,
// };

use libp2p::{identity, noise, tcp, yamux};


use libp2p::{
    core::upgrade,
    floodsub::{Floodsub, FloodsubEvent, Topic},
    
   
     PeerId, Transport,
};
use libp2p::mdns::{Mdns, MdnsEvent};
use libp2p::swarm::NetworkBehaviour;

use libp2p::Swarm;
use tokio::sync::mpsc;
use std::error::Error;

#[derive(NetworkBehaviour)]
#[behaviour(event_process = false)]
struct BlockchainBehaviour {
    floodsub: Floodsub,
    mdns: mdns::async_io::Behaviour,
}

pub struct Network {
    swarm: Swarm<BlockchainBehaviour>,
}
impl BlockchainBehaviour {
    // Implement necessary methods here
}
impl Network {

    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let id_keys = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(id_keys.public());
        
        let transport = tcp::async_io::Transport::new(tcp::Config::default().nodelay(true))
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseAuthenticated::xx(&id_keys).into_authenticated())
            .multiplex(yamux::Config::default())
            .boxed();

        let behaviour = BlockchainBehaviour {
            floodsub: Floodsub::new(peer_id),
            mdns: mdns::async_io::Behaviour::new(mdns::Config::default())?,
        };

        let swarm = Swarm::with_async_std_executor(transport, behaviour, peer_id);

        Ok(Network { swarm })
    }

    pub async fn run(&mut self) {
        loop {
            match self.swarm.next().await {
                Some(event) => {
                    // Handle network events
                }
                None => break,
            }
        }
    }
}