use std::io;

use clap::{Arg, Command};

struct ApiClient {
    server: String,
    client: reqwest::blocking::Client,
}

impl ApiClient {
    fn post_logs(&self, req: &api::logs::post::Request) -> reqwest::Result<()> {
        self.client
            .post(&format!("http://{}/logs", &self.server))
            .json(req)
            .send()
            .map(|_| ())
    }

    fn get_logs(&self) -> reqwest::Result<api::logs::get::Response> {
        self.client
            .get(&format!("http://{}/logs", &self.server))
            .send()?
            .json()
    }
}

fn do_post_csv(api_client: &ApiClient) {
    let reader = csv::Reader::from_reader(io::stdin());
    for log in reader.into_deserialize::<api::logs::post::Request>() {
        let log = match log {
            Ok(log) => log,
            Err(e) => {
                eprintln!("[WARN] failed to parse a line, skipping: {}", e);
                continue;
            }
        };
        api_client.post_logs(&log).expect("api request failed");
    }
}

fn do_get_json(api_client: &ApiClient) {
    let res = api_client.get_logs().expect("api request failed");
    let json_str = serde_json::to_string(&res).unwrap();
    println!("{}", json_str);
}

fn main() {
    let opts = Command::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("SERVER")
                .short('s')
                .long("server")
                .value_name("URL")
                .help("server url")
                .takes_value(true),
        )
        .subcommand(Command::new("post").about("post logs, taking input from stdin"))
        .subcommand(
            Command::new("get").about("get logs").arg(
                Arg::new("FORMAT")
                    .help("log format")
                    .short('f')
                    .long("format")
                    .takes_value(true)
                    .possible_values(&["csv", "json"])
                    .ignore_case(true),
            ),
        );
    let matches = opts.get_matches();

    let server = matches
        .value_of("SERVER")
        .unwrap_or("localhost:3000")
        .into();
    let client = reqwest::blocking::Client::new();
    let api_client = ApiClient { server, client };
    match matches.subcommand() {
        Some(("get", sub_match)) => {
            let format = sub_match.value_of("FORMAT").unwrap();
            match format {
                "csv" => unimplemented!(),
                "json" => do_get_json(&api_client),
                _ => panic!(),
            }
        }
        Some(("post", _)) => do_post_csv(&api_client),
        _ => unreachable!(),
    }
}
