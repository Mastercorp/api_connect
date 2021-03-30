use clap::{App, Arg};
use configparser::ini::Ini;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use std::{collections::HashMap, io::Read};

struct RestClient<'a, 'b, 'c, 'd> {
    client: Client,
    hostname: &'a str,
    token: &'b str,
    header: &'c str,
    username: &'d str,
}
fn main() {
    let args = App::new("Crypto Info")
        .version("0.1")
        .about("Git checker")
        .arg(
            Arg::with_name("endpoint")
                .help("name of the endpoint")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let config_map = read_config();
    let hostname = String::from(config_map["server"]["hostname"].clone().unwrap());
    let username = String::from(config_map["server"]["username"].clone().unwrap());
    let token = String::from(config_map["server"]["token"].clone().unwrap());
 
    let client = RestClient {
        client: Client::new(),
        hostname: &hostname,
        token: &token,
        header: "rust_info",
        username: &username,
    };

    // read in the endpoint as an argument to the call of the programm. (cargo run "users/YourAccount")
    let endpoint = args.value_of("endpoint").unwrap();
    let response = get_to_api(&client, &endpoint);

    // here you could do something with the response. It is currently a String, but maybe its better to read it in as json.

    // just as a general info how a post request would look like, with a param. In this case, a gist would be created.
    //let post_param = json!({"files":{"tesythuis":{"content":"Python requests has 3 parameters: 1)Request URL\n 2)Header Fields\n 3)Parameter \n4)Request body"}}});
    //post_to_api(&client, "gists", post_param);

}

fn read_config() -> HashMap<String, HashMap<String, Option<String>>> {
    let mut config = Ini::new();
    // You can easily load a file to get a clone of the map:
    let map = config.load("src/config.cfg").unwrap();
    map
}

fn get_to_api(rest_client: &RestClient, endpoint: &str) -> String {
    let mut resp = rest_client
        .client
        .get(format!("{}{}", rest_client.hostname, endpoint))
        .basic_auth(rest_client.username, Some(rest_client.token))
        .header(USER_AGENT, rest_client.header)
        .send()
        .unwrap();

    let mut body = String::new();
    resp.read_to_string(&mut body).unwrap();

    println!("{:?}", body);
    println!("{:?}", resp);

    body
}

fn post_to_api(rest_client: &RestClient, endpoint: &str, params: serde_json::Value) -> String {
    let mut resp = rest_client
        .client
        .post(format!("{}{}", rest_client.hostname, endpoint))
        .basic_auth(rest_client.username, Some(rest_client.token))
        .json(&params)
        .header(USER_AGENT, rest_client.header)
        .send()
        .unwrap();

    let mut body = String::new();
    resp.read_to_string(&mut body).unwrap();

    println!("{:?}", body);
    println!("{:?}", resp);

    body
}