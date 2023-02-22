use std::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(default_value = "1")]
    count: u32,
}

#[derive(serde::Deserialize)]
struct FortuneCookie {
    cookie: InnerFortune,
}

#[derive(serde::Deserialize)]
struct InnerFortune {
    fortune: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    let count = args.count;

    for _ in 0..count {
        let url = "https://digital-fortune-cookies-api.herokuapp.com/fortune";
        let response = reqwest::get(url).await?;
        let body = response.text().await?;
        let fortune_cookie: FortuneCookie = match serde_json::from_str(&body) {
            Ok(cookie) => cookie,
            Err(e) => return Err(e.into()),
        };
        println!("Fortune: {}", fortune_cookie.cookie.fortune);
    }

    Ok(())
}
