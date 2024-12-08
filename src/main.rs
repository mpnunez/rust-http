use tokio;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let result = make_request("http://example.com").await?;
    println!("Response: {}", result);
    Ok(())
}

async fn make_request(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await?
        .error_for_status()?;

    let body = response.text().await?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_make_good_request(){
        let result = make_request("http://example.com").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_make_bad_request(){
        let result = make_request("http://does-not-exist.com").await;
        assert!(result.is_err());
    }
}
