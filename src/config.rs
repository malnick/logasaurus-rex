extern crate clap;

use clap::ArgMatches;
use std::string::String;

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub elastic_host: String,
    pub elastic_port: String,
    pub elastic_index: String,
    pub match_query: String,
    // seconds
    pub sync_interval: u64,
    pub sync_depth: u64,
    pub sync_count: u64,
    pub sync_start: u64,
}

impl Config {
    pub fn new(a: ArgMatches) -> Config {
        let mut c = Config {
            elastic_host: "127.0.0.1".to_string(),
            elastic_port: "9300".to_string(),
            elastic_index: "filebeat-*".to_string(),
            match_query: "".to_string(),
            sync_interval: 5,
            sync_depth: 300,
            sync_count: 1,
            sync_start: 0,
        };

        if let Some(host) = a.value_of("elastic-host") {
            c.elastic_host = host.to_string()
        }

        if let Some(port) = a.value_of("elastic-port") {
            c.elastic_port = port.to_string()
        }

        if let Some(index) = a.value_of("elastic-index") {
            c.elastic_index = index.to_string()
        }

        if let Some(match_query) = a.value_of("match") {
            c.match_query = match_query.to_string()
        }

        if let Some(count) = a.value_of("count") {
            c.sync_count = count.parse().unwrap()
        }

        if let Some(int) = a.value_of("sync-interval") {
            c.sync_interval = int.parse().unwrap()
        }

        if let Some(depth) = a.value_of("sync-depth") {
            c.sync_depth = depth.parse().unwrap()
        }

        if let Some(start) = a.value_of("sync-start") {
            c.sync_start = start.parse().unwrap()
        }

        c
    }
}
