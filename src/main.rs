use std::fmt;
use std::str::FromStr;

use anyhow::Result;
use futures_lite::StreamExt;
use iroh::protocol::Router;
use iroh::{Endpoint,NodeAddr, SecretKey};
use iroh_gossip::net::{Event,Gossip, GossipEvent, GossipReceiver};
use iroh_gossip::proto::TopicId;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<()> {
    let secret_key: SecretKey = SecretKey::generate(rand::rngs::OsRng);
    println!("> our secret key: {}", secret_key);

    let endpoint: Endpoint = Endpoint::builder()
        .secret_key(secret_key)
        .discovery_n0()
        .bind()
        .await?;
 
    println!("> our node id: {}", endpoint.node_id());
    let gossip: Gossip = Gossip::builder().spawn(endpoint.clone()).await?;

    let router: Router = Router::builder(endpoint.clone())
        .accept(iroh_gossip::ALPN, gossip.clone())
        .spawn()
        .await?;

    let topic: TopicId = TopicId::from_bytes(rand::random());
    let peers = vec![];
    let (sender, receiver) = gossip.subscribe(topic,peers.clone())?.split();
    
    tokio::spawn(subscribe_loop(receiver));

    let ticket = Ticket {
        topic,
        peers:vec![],
    };

    println!("> ticket to join us: {ticket}");    

    sender.broadcast("sup".into()).await?;


    router.shutdown().await?;

    Ok(())
}
    async fn subscribe_loop(mut receiver: GossipReceiver) -> Result<()>{
        while let Some(event) = receiver.try_next().await?{
            if let Event::Gossip(gossip_event) = event {
                match gossip_event {
                    GossipEvent::Received(message) => println!("got message: {:?}", &message),
                    _ => println!("got event: {:?}", &gossip_event),
                }
            }
        
        }
        Ok(())
    }

    #[derive(Debug,Serialize,Deserialize)]
    struct Ticket {
        topic: TopicId,
        peers:Vec<NodeAddr>,
    }

    impl Ticket {
        fn from_bytes(bytes: &[u8]) -> Result<Self> {
            serde_json::from_slice(bytes).map_err(Into::into)
        }
        pub fn to_bytes(&self) -> Vec<u8> {
            serde_json::to_vec(self).expect("serde_json::to_vec is infallible")
        }
    }

    impl fmt::Display for Ticket {
        fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
            let mut text = data_encoding::BASE32_NOPAD.encode(&self.to_bytes()[..]);
            text.make_ascii_lowercase();
            write!(f,"{}",text)
        }
    }

    impl FromStr for Ticket {
        type Err = anyhow::Error;
        fn from_str(s: &str) -> Result<Self,Self::Err> {
            let bytes = data_encoding::BASE32_NOPAD.decode(s.to_ascii_uppercase().as_bytes())?;
            Self::from_bytes(&bytes)
        }
    }
