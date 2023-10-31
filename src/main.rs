use std::time::Duration;

use reqwest::ClientBuilder;
use weather_api::{gen_schema, swagger};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = ClientBuilder::new()
        .gzip(true)
        .deflate(true)
        .user_agent("dev weather client - github.com/leb-kuchen")
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(10))
        .build()?;
    let res = client
        .get("https://api.met.no/weatherapi/locationforecast/2.0/complete?lat=50.92&lon=6.96&altitude=40")
        .header("Accept", "application/json")
        .send()
        .await?;
    println!("{}", res.status());
    println!("{:#?}", res.headers());
    let weather = res.json::<swagger::Metjsonforecast>().await?;
    println!("{:#?}", weather);
    gen_schema();
    Ok(())
}
