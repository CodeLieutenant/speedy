use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IPerf3 {
    pub start: Start,
    pub intervals: Vec<Interval>,
    pub end: End,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Start {
    pub connected: Vec<Connected>,
    pub version: String,
    #[serde(rename = "system_info")]
    pub system_info: String,
    pub timestamp: Timestamp,
    #[serde(rename = "connecting_to")]
    pub connecting_to: ConnectingTo,
    pub cookie: String,
    #[serde(rename = "tcp_mss_default")]
    pub tcp_mss_default: i64,
    #[serde(rename = "target_bitrate")]
    pub target_bitrate: i64,
    #[serde(rename = "fq_rate")]
    pub fq_rate: i64,
    #[serde(rename = "sock_bufsize")]
    pub sock_bufsize: i64,
    #[serde(rename = "sndbuf_actual")]
    pub sndbuf_actual: i64,
    #[serde(rename = "rcvbuf_actual")]
    pub rcvbuf_actual: i64,
    #[serde(rename = "test_start")]
    pub test_start: TestStart,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connected {
    pub socket: i64,
    #[serde(rename = "local_host")]
    pub local_host: String,
    #[serde(rename = "local_port")]
    pub local_port: i64,
    #[serde(rename = "remote_host")]
    pub remote_host: String,
    #[serde(rename = "remote_port")]
    pub remote_port: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timestamp {
    pub time: String,
    pub timesecs: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectingTo {
    pub host: String,
    pub port: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestStart {
    pub protocol: String,
    #[serde(rename = "num_streams")]
    pub num_streams: i64,
    pub blksize: i64,
    pub omit: i64,
    pub duration: i64,
    pub bytes: i64,
    pub blocks: i64,
    pub reverse: i64,
    pub tos: i64,
    #[serde(rename = "target_bitrate")]
    pub target_bitrate: i64,
    pub bidir: i64,
    pub fqrate: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interval {
    pub streams: Vec<Stream>,
    pub sum: Sum,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    pub socket: i64,
    pub start: f64,
    pub end: f64,
    pub seconds: f64,
    pub bytes: i64,
    #[serde(rename = "bits_per_second")]
    pub bits_per_second: f64,
    pub sender: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sum {
    pub start: f64,
    pub end: f64,
    pub seconds: f64,
    pub bytes: i64,
    #[serde(rename = "bits_per_second")]
    pub bits_per_second: f64,
    pub sender: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct End {
    pub streams: Vec<Stream2>,
    #[serde(rename = "sum_sent")]
    pub sum_sent: SumSent,
    #[serde(rename = "sum_received")]
    pub sum_received: SumReceived,
    #[serde(rename = "cpu_utilization_percent")]
    pub cpu_utilization_percent: CpuUtilizationPercent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stream2 {
    pub sender: Sender,
    pub receiver: Receiver,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sender {
    pub socket: i64,
    pub start: i64,
    pub end: f64,
    pub seconds: f64,
    pub bytes: i64,
    #[serde(rename = "bits_per_second")]
    pub bits_per_second: Option<f64>,
    #[serde(rename = "max_snd_cwnd")]
    pub max_snd_cwnd: Option<i64>,
    #[serde(rename = "max_snd_wnd")]
    pub max_snd_wnd: Option<i64>,
    #[serde(rename = "max_rtt")]
    pub max_rtt: Option<i64>,
    #[serde(rename = "min_rtt")]
    pub min_rtt: i64,
    #[serde(rename = "mean_rtt")]
    pub mean_rtt: i64,
    pub sender: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Receiver {
    pub socket: i64,
    pub start: i64,
    pub end: f64,
    pub seconds: f64,
    pub bytes: i64,
    #[serde(rename = "bits_per_second")]
    pub bits_per_second: f64,
    pub sender: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SumSent {
    pub start: i64,
    pub end: f64,
    pub seconds: f64,
    pub bytes: i64,
    #[serde(rename = "bits_per_second")]
    pub bits_per_second: f64,
    pub sender: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SumReceived {
    pub start: i64,
    pub end: f64,
    pub seconds: f64,
    pub bytes: i64,
    #[serde(rename = "bits_per_second")]
    pub bits_per_second: f64,
    pub sender: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuUtilizationPercent {
    #[serde(rename = "host_total")]
    pub host_total: f64,
    #[serde(rename = "host_user")]
    pub host_user: f64,
    #[serde(rename = "host_system")]
    pub host_system: f64,
    #[serde(rename = "remote_total")]
    pub remote_total: f64,
    #[serde(rename = "remote_user")]
    pub remote_user: f64,
    #[serde(rename = "remote_system")]
    pub remote_system: f64,
}
