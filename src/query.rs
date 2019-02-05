extern crate clap;
extern crate elastic;
extern crate tokio;

use super::config;
use config::Config;
use elastic::prelude::*;
use serde_json::to_string_pretty;
use std::error::Error;
use std::{thread, time};

#[derive(Deserialize, Debug)]
struct MyResponse {
    pub message: String,
}

fn query(c: &Config) {
    let client = SyncClientBuilder::new()
        .base_url(format!("{}:{}", c.elastic_host, c.elastic_port))
        .build()
        .unwrap();

    let j = json!({
        "from": 0,
        "size": c.sync_count,
        "sort": {
           "@timestamp": "asc",
        },
        "query": {
            "bool": {
                "filter": {
                    "range": {
                        "@timestamp": {
                            "gte": format!("now-{}s", c.sync_depth),
                            "lte": format!("now-{}s", c.sync_start),
                        }
                    }
                },
                "must": {
                    "match": {
                        "message": c.match_query
                    }
                }
            }
        }
    });

    //    println!("request: {}", to_string_pretty(&j).unwrap());

    let res = client
        .search::<MyResponse>()
        .index(c.elastic_index.clone())
        .body(j)
        .send()
        .unwrap();

    for hit in res.hits() {
        if let Some(h) = hit.document() {
            println!("{}", h.message)
        }
    }
}

pub fn run(c: Config) -> Result<(), Box<Error>> {
    let int = time::Duration::from_secs(c.sync_interval);

    let mut sync_count = 0;
    let mut loc_c = &mut c.clone();
    while sync_count >= 0 {
        if sync_count > 0 {
            loc_c.sync_depth = loc_c.sync_depth + loc_c.sync_interval;
            loc_c.sync_start = loc_c.sync_start + loc_c.sync_interval;
        } else {
            loc_c.sync_depth = loc_c.sync_start + loc_c.sync_depth
        }

        query(loc_c);
        thread::sleep(int);
        sync_count = sync_count + 1;
    }

    Ok(())
}
