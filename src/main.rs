mod cli;
mod influxdb;
mod iperf3;
mod models;
mod timetable;

#[tokio::main]
async fn main() {
    match cli::execute().await {
        Ok(_) => println!("Job finished"),
        Err(e) => eprintln!("{e}"),
    }
}
