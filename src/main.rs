mod cli;
mod influxdb;
mod iperf3;
mod models;
mod timetable;

#[tokio::main]
async fn main() {
    match cli::execute().await {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}
