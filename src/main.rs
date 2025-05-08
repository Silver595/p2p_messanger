use anyhow::Result;
use futures_lite::StreamExt;
use iroh::protocol::Router;
use iroh::{Endpoint, SecretKey};
use iroh_gossip::net::{Event,Gossip, GossipEvent, GossipReceiver};
use iroh_gossip::proto::TopicId;

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

    let id: TopicId = TopicId::from_bytes(rand::random());
    let peer_ids = vec![];
    let (sender, receiver) = gossip.subscribe(id,peer_ids)?.split();
    
    tokio::spawn(subscribe_loop(receiver));

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
