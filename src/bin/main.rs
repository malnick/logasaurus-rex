extern crate clap;
extern crate logasaurus_rex;

use clap::{App, Arg};
use logasaurus_rex::config;
use logasaurus_rex::query;

fn main() {
    println!("{}", steg());

    let matches = App::new("logasaurus-rex")
        .version("1.0")
        .author("Jeff Malnick <malnick@gmail.com>")
        .about("tail logs like a boss")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("elastic-host")
                .long("host")
                .help("Sets elasticsearch host")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("elastic-port")
                .long("port")
                .help("Sets elasticsearch port")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("elastic-index")
                .long("index")
                .help("Sets elasticsearch index")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("match")
                .short("m")
                .long("match")
                .help("match query for search")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .help("how many matches to return")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("sync-interval")
                .short("i")
                .long("sync-interval")
                .help("how often to sync the lookup, in seconds")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("sync-depth")
                .short("d")
                .long("sync-depth")
                .help("how deep to sync the query, in seconds")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("sync-start")
                .short("s")
                .long("sync-start")
                .help("how long ago to start the sync for the query, in seconds")
                .takes_value(true),
        )
        .get_matches();

    let config = config::Config::new(matches.clone());

    if let Some(match_query) = matches.value_of("match") {
        println!("making match query for {}", match_query);

        query::run(config).unwrap()
    }
}

fn steg() -> String {
    let steg = r#"
                        .       .                             
                       / '.   .' \                           
               .---.  <    > <    >  .---.                  
               |    \  \ - ~ ~ - /  /    |                 
               ~-..-~             ~-..-~                     
            \~~~\.'                    './~~~/              
  .-~~^-.    \__/                        \__/                 
.'  O    \     /               /       \  \                   
(_____'    \._.'              |         }  \/~~~/             
  ----.         /       }     |        /    \__/              
      \-.      |       /      |       /      \.,~~|           
          ~-.__|      /_ - ~ ^|      /- _     \..-'   f: f:   
               |     /        |     /     ~-.     -. _||_||_  
               |_____|        |_____|         ~ - . _ _ _ _ _>
██╗      ██████╗  ██████╗  █████╗ ███████╗ █████╗ ██╗   ██╗██████╗ ██╗   ██╗███████╗
██║     ██╔═══██╗██╔════╝ ██╔══██╗██╔════╝██╔══██╗██║   ██║██╔══██╗██║   ██║██╔════╝
██║     ██║   ██║██║  ███╗███████║███████╗███████║██║   ██║██████╔╝██║   ██║███████╗
██║     ██║   ██║██║   ██║██╔══██║╚════██║██╔══██║██║   ██║██╔══██╗██║   ██║╚════██║
███████╗╚██████╔╝╚██████╔╝██║  ██║███████║██║  ██║╚██████╔╝██║  ██║╚██████╔╝███████║
╚══════╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚══════╝
"#;
    steg.to_string()
}
