use dotenv;
use json::JsonValue;
use std::env;
use reqwest::header::AUTHORIZATION;

async fn get_url(url: &str, token: String) -> String {
    let client = reqwest::Client::new();
    let resp: Result<String, reqwest::Error> = client.get(url)
        .header(AUTHORIZATION, token)
        .send()
        .await.unwrap().text().await;
    resp.ok().expect("error")
}

fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn menu() {
    println!("1. Give me a joke !");
    println!("2. Exit");
}

async fn get_joke() -> JsonValue {
    dotenv::dotenv().ok();
    let response: String = get_url("https://www.blagues-api.fr/api/random", "Bearer ".to_string() + &env::var("API_KEY").unwrap()).await;
    let parsed: JsonValue = json::parse(&response).unwrap();
    parsed
}

#[tokio::main]
async fn main() {
    menu();
    while get_user_input() == "1" {
        let joke = get_joke().await;
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        print!("{}[2J", 27 as char);
        println!("{}", joke["joke"].to_string());
        println!("{}", joke["answer"].to_string());
        menu();
    }
}
