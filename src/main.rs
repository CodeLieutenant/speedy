mod cli;
mod influxdb;
mod iperf3;
mod models;
mod timetable;

#[tokio::main]
async fn main() {
    cli::execute().await;
}
