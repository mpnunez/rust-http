use tokio;
use reqwest;
use async_trait::async_trait;

#[derive(Debug)]
struct NetworkError;

impl From<reqwest::Error> for NetworkError {
    fn from(e: reqwest::Error) -> NetworkError {
        println!("Application error: {e}");
        NetworkError
    }
}

#[async_trait]
trait HttpBodyGetter {
    async fn get_http_response_body(&self, url: &str) -> Result<String, NetworkError>;
}

#[async_trait]
impl HttpBodyGetter for reqwest::Client {
    async fn get_http_response_body(&self, url: &str) -> Result<String, NetworkError> {
        Ok(self.get(url)
            .send().await?
            .error_for_status()?
            .text().await?)
    }
}


#[tokio::main]
async fn main() -> Result<(), NetworkError> {
    let client = reqwest::Client::new();
    let result = returns_html("http://example2222.com", &client).await?;
    println!("Response: {}", result);
    Ok(())
}

async fn returns_html(url: &str, client: &impl HttpBodyGetter) -> Result<bool, NetworkError> {
    let body = client.get_http_response_body(url).await?;
    Ok(body.starts_with("<!doctype html>"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{rstest,fixture};

    #[fixture]
    pub fn client_for_test() -> impl HttpBodyGetter {
        reqwest::Client::new()
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_html(client_for_test: impl HttpBodyGetter){
        let result = returns_html("http://example.com", &client_for_test).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[rstest]
    #[tokio::test]
    async fn test_get_non_html(client_for_test: impl HttpBodyGetter){
        let result = returns_html("https://ringsdb.com/api/public/card/01005", &client_for_test).await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[rstest]
    #[tokio::test]
    async fn test_bad_request(client_for_test: impl HttpBodyGetter){
        let result = returns_html("http://does-not-exist.com", &client_for_test).await;
        assert!(result.is_err());
    }
}
