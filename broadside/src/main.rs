use broadside_net::Node;

#[tokio::main]
async fn main() {
    let (mut server, server_addr) = Node::host().await.unwrap();
    println!("Bound server at {server_addr:?}");
    let mut client = Node::connect(server_addr).await.unwrap();
    server.write(&[1,2,3,4,5,6,7,8]).await.unwrap();
    let recv = client.read().await.unwrap();
    println!("Received {recv:?}");
}
