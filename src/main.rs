use tokio;
use reqwest;
use async_trait::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

#[derive(Debug)]
struct NetworkError;

// Enables reqwest::Error to raise a NetworkError using the ? operator
impl From<reqwest::Error> for NetworkError {
    fn from(e: reqwest::Error) -> NetworkError {
        println!("Application error: {e}");
        NetworkError
    }
}

// Order of macros matters. See https://medium.com/vortechsa/mocking-in-async-rust-248b012c5e99
#[cfg_attr(test, automock)]
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
    let result = returns_html("http://example.com", &client).await?;
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
        let mut mock_client = MockHttpBodyGetter::new();
        mock_client.expect_get_http_response_body()
            .returning(
                |url|
                match url {
                    "http://example.com" => Ok("<!doctype html></html>".to_string()),
                    "https://ringsdb.com/api/public/card/01005" => Ok("{name\":\"Legolas}".to_string()),
                    _ => Err(NetworkError)
                }
            );
        return mock_client;
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
