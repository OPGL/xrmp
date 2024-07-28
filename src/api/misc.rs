use reqwest::blocking::Client;

pub fn read_from_web(url: String) -> anyhow::Result<String> {
    let client = Client::new();
    let response = client.get(url).header("User-Agent", "xrmp/1.0").send()?;
    Ok(response.text()?)
}
