use std::fmt;
use std::net::Ipv4Addr;
use std::str::FromStr;
use xmlrpc::{Request, Value};

#[derive(Debug)]
pub struct RTorrent {
    url: String,
}

#[derive(Debug, Clone)]
pub struct Torrent {
    hash: String,
    name: String,
    path: String,
    size: i64,
    label: String,
    // completed: bool,
    ratio: f64,
}

#[derive(Debug)]
pub struct Status {
    completed: bool,
    completed_bytes: i64,
    down_rate: i64,
    up_rate: i64,
    ratio: f64,
}

#[derive(Debug)]
pub struct File {
    path: String,
    size: i64,
}


impl RTorrent {
    pub fn new(url: String) -> Self {
        RTorrent { url }
    }

    pub fn ip(&self) -> Ipv4Addr {
        Ipv4Addr::from_str(
            Request::new("network.bind_address")
                .call_url(&self.url)
                .unwrap()
                .as_str()
                .unwrap(),
        )
        .unwrap()
    }

    pub fn name(&self) -> String {
        Request::new("system.hostname")
            .call_url(&self.url)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
    }

    pub fn down_total(&self) -> i64 {
        Request::new("throttle.global_down.total")
            .call_url(&self.url)
            .unwrap()
            .as_i64()
            .unwrap()
    }

    pub fn down_rate(&self) -> i64 {
        Request::new("throttle.global_down.rate")
            .call_url(&self.url)
            .unwrap()
            .as_i64()
            .unwrap()
    }

    pub fn up_total(&self) -> i64 {
        Request::new("throttle.global_up.total")
            .call_url(&self.url)
            .unwrap()
            .as_i64()
            .unwrap()
    }

    pub fn up_rate(&self) -> i64 {
        Request::new("throttle.global_up.rate")
            .call_url(&self.url)
            .unwrap()
            .as_i64()
            .unwrap()
    }

    pub fn get_torrents(&self, view: String) -> Vec<Torrent> {
        Request::new("d.multicall2").arg("")
            .arg(view)
            .arg("d.hash=")
            .arg("d.name=")
            .arg("d.base_path=")
            .arg("d.size_bytes=")
            .arg("d.custom1=")
            .arg("d.ratio=")
            .call_url(&self.url)
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                Torrent {
                    hash: x[0].as_str().unwrap().to_string(),
                    name: x[1].as_str().unwrap().to_string(),
                    path: x[2].as_str().unwrap().to_string(),
                    size: x[3].as_i64().unwrap(),
                    label: x[4].as_str().unwrap().to_string(),
                    ratio: x[5].as_i64().unwrap() as f64 / 1000f64,
                }
            })
            .collect::<Vec<Torrent>>()
    }

    pub fn get_status(&self, torrent: Torrent) -> Status {
        Status {
            completed: Request::new("d.complete").arg(torrent.hash.clone()).call_url(&self.url).unwrap().as_i64().unwrap() > 0,
            completed_bytes: Request::new("d.completed_bytes").arg(torrent.hash.clone()).call_url(&self.url).unwrap().as_i64().unwrap(),
            down_rate: Request::new("d.down.rate").arg(torrent.hash.clone()).call_url(&self.url).unwrap().as_i64().unwrap(),
            up_rate: Request::new("d.up.rate").arg(torrent.hash.clone()).call_url(&self.url).unwrap().as_i64().unwrap(),
            ratio: Request::new("d.down.rate").arg(torrent.hash).call_url(&self.url).unwrap().as_i64().unwrap() as f64 / 1000f64,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
