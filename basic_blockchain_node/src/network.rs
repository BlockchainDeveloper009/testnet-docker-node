use libp2p::{
    core::upgrade,
    floodsub::{Floodsub, FloodsubEvent, Topic},
    mdns::{Mdns, MdnsEvent},
    swarm::{NetworkBehaviourEventProcess, Swarm, SwarmBuilder},
    NetworkBehaviour, PeerId, Transport,
};
use tokio::sync::mpsc;
use std::error::Error;

#[derive(NetworkBehaviour)]
struct BlockchainBehaviour {
    floodsub: Floodsub,
    mdns: Mdns,
}

pub struct Network {
    swarm: Swarm<BlockchainBehaviour>,
}

impl Network {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let id_keys = libp2p::identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(id_keys.public());
        let transport = libp2p::tcp::TokioTcpConfig::new()
            .nodelay(true)
            .upgrade(upgrade::Version::V1)
            .authenticate(libp2p::noise::NoiseConfig::xx(id_keys).into_authenticated())
            .multiplex(libp2p::yamux::YamuxConfig::default())
            .boxed();

        let mut behaviour = BlockchainBehaviour {
            floodsub: Floodsub::new(peer_id),
            mdns: Mdns::new(Default::default()).await?,
        };

        let swarm = SwarmBuilder::new(transport, behaviour, peer_id)
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build();

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