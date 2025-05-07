use anyhow::Result;
use iroh::protocol::Router;
use iroh::{Endpoint, SecretKey};
use iroh_gossip::net::Gossip;
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
    let (sender, _reciver) = gossip.subscribe(id,peer_ids)?.split();
    sender.broadcast("sup".into()).await?;


    router.shutdown().await?;

    Ok(())
}