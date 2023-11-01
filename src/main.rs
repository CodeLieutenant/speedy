mod cli;
mod influxdb;
mod iperf3;
mod models;

#[tokio::main]
async fn main() {
    cli::execute().await;
}
