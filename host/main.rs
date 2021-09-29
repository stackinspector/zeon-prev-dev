use tokio::signal;

#[tokio::main]
async fn main() {
    signal::ctrl_c().await.unwrap();
}
