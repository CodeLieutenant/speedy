use influxdb::InfluxDbWriteable;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    InfluxDB(#[from] influxdb::Error),
}

#[derive(InfluxDbWriteable, Clone, Debug)]
pub struct Speed {
    time: influxdb::Timestamp,
    #[influxdb(tag)]
    direction: String,
    speed: u64, // bits per second
}

#[derive(Debug)]
pub struct Client {
    inner: influxdb::Client,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Download,
    Upload,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match *self {
            Direction::Download => "down".to_string(),
            Direction::Upload => "up".to_string(),
        }
    }
}

impl Speed {
    pub fn new(time: time::OffsetDateTime, direction: Direction, speed: u64) -> Self {
        Self {
            time: influxdb::Timestamp::Seconds(time.unix_timestamp() as u128),
            direction: direction.to_string(),
            speed,
        }
    }
}

impl Client {
    #[inline]
    pub fn new(addr: impl AsRef<str>, bucket: impl AsRef<str>, token: impl AsRef<str>) -> Self {
        Self {
            inner: influxdb::Client::new(addr.as_ref(), bucket.as_ref()).with_token(token.as_ref()),
        }
    }

    #[inline]
    pub async fn insert_multiple(&self, speeds: impl Iterator<Item = Speed>) -> Result<(), Error> {
        self.inner
            .query(
                speeds
                    .map(|item| item.into_query("network_speeds"))
                    .collect::<Vec<_>>(),
            )
            .await?;
        Ok(())
    }
}
