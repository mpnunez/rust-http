use tokio;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let result = returns_html("http://example.com").await?;
    println!("Response: {}", result);
    Ok(())
}

async fn returns_html(url: &str) -> Result<bool, reqwest::Error> {
    
    let client = reqwest::Client::new();
    let body = client.get(url)
            .send().await?
            .error_for_status()?
            .text().await?;
    Ok(body.starts_with("<!doctype html>"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_html(){

        let result = returns_html("http://example.com").await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_get_non_html(){

        let result = returns_html("https://ringsdb.com/api/public/card/01005").await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_bad_request(){
        let result = returns_html("http://does-not-exist.com").await;
        assert!(result.is_err());
    }
}
