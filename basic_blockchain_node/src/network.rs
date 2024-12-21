use libp2p::{
    core::upgrade,
    floodsub::{Floodsub, FloodsubEvent, Topic},
    gossipsub::{Gossipsub, GossipsubConfig, GossipsubEvent, MessageAuthenticity},
    identity, noise, tcp, yamux,
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviour,
    PeerId, Swarm,
};
use std::error::Error;
use futures::StreamExt;
use async_std::task;

// Define a custom NetworkBehaviour to combine multiple protocols (Floodsub, Gossipsub, Mdns).
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "BlockchainBehaviourEvent")]
pub struct BlockchainBehaviour {
    // Floodsub: A simple publish-subscribe protocol.
    pub floodsub: Floodsub,
    // Mdns: Enables peer discovery in a local network using mDNS.
    pub mdns: Mdns,
    // Gossipsub: A more advanced pub-sub protocol with topic-based subscriptions.
    pub gossipsub: Gossipsub,
}

// Define a custom enum to handle events from the NetworkBehaviour.
#[derive(Debug)]
pub enum BlockchainBehaviourEvent {
    Floodsub(FloodsubEvent), // Events from Floodsub protocol.
    Mdns(MdnsEvent),         // Events from Mdns protocol.
    Gossipsub(GossipsubEvent), // Events from Gossipsub protocol.
}

// Implement conversions from individual protocol events 
// to the unified BlockchainBehaviourEvent.
impl From<FloodsubEvent> for BlockchainBehaviourEvent {
    fn from(event: FloodsubEvent) -> Self {
        BlockchainBehaviourEvent::Floodsub(event)
    }
}

impl From<MdnsEvent> for BlockchainBehaviourEvent {
    fn from(event: MdnsEvent) -> Self {
        BlockchainBehaviourEvent::Mdns(event)
    }
}

impl From<GossipsubEvent> for BlockchainBehaviourEvent {
    fn from(event: GossipsubEvent) -> Self {
        BlockchainBehaviourEvent::Gossipsub(event)
    }
}

// Define the Network structure to encapsulate the Swarm 
and its associated logic.
pub struct Network {
    swarm: Swarm<BlockchainBehaviour>, 
    // Swarm that drives the network behaviour.
}

impl Network {
    // Create a new Network instance.
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        // Generate a new Ed25519 keypair for the peer.
        let id_keys = identity::Keypair::generate_ed25519();
        // Derive the PeerId from the public key.
        let peer_id = PeerId::from(id_keys.public());

        println!("Local Peer ID: {}", peer_id); 
        // Print the PeerId to the console.

        // Build the transport stack: TCP, Noise (secure handshake), and Yamux (multiplexing).
        let transport = tcp::async_io::Transport::new(tcp::Config::default().nodelay(true))
            .upgrade(upgrade::Version::V1) // Use V1 for protocol upgrades.
            .authenticate(noise::NoiseAuthenticated::xx(&id_keys).unwrap()) // Secure communication using Noise protocol.
            .multiplex(yamux::Config::default()) // Multiplex multiple streams over a single connection.
            .boxed(); // Boxed to make it easier to pass around.

        // Initialize the Floodsub protocol for simple publish-subscribe messaging.
        let floodsub = Floodsub::new(peer_id);
        // Initialize the Mdns protocol for local peer discovery.
        let mdns = Mdns::new(Default::default()).await?;
        // Initialize the Gossipsub protocol for topic-based pub-sub messaging.
        let gossipsub_config = GossipsubConfig::default();
        let gossipsub = Gossipsub::new(MessageAuthenticity::Signed(id_keys.clone()), gossipsub_config)?;

        // Combine the behaviours into a single NetworkBehaviour.
        let behaviour = BlockchainBehaviour {
            floodsub,
            mdns,
            gossipsub,
        };

        // Create the Swarm, which drives the behaviour and handles events.
        let swarm = Swarm::with_executor(transport, behaviour, peer_id, task::spawn);

        Ok(Network { swarm }) // Return the Network instance.
    }

    // Start the network event loop to process incoming and outgoing events.
    pub async fn run(&mut self) {
        loop {
            match self.swarm.next().await {
                Some(event) => println!("Event: {:?}", event), // Print the event to the console.
                None => break, // Break the loop if there are no more events.
            }
        }
    }
}

// The main entry point of the application.
#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a new network instance.
    let mut network = Network::new().await?;
    // Run the network event loop.
    network.run().await;
    Ok(())
}
