use tokio;
use reqwest::{Client,Response,Error};
use async_trait::async_trait;

#[async_trait]
trait HttpGetter {  // Need to mock this trait for unit tests
    async fn get_http_response(&self, url: &str) -> Result<Response, Error>;
}

#[async_trait]
impl HttpGetter for Client {
    async fn get_http_response(&self, url: &str) -> Result<Response, Error> {
        self.get(url).send().await
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();
    let result = make_request("http://example.com", &client).await?;
    println!("Response: {}", result);
    Ok(())
}

async fn make_request(url: &str, client: &impl HttpGetter) -> Result<String, Error> {
    let response = client.get_http_response(url).await?.error_for_status()?;
    let body = response.text().await?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{rstest,fixture};

    #[fixture]
    pub fn client_for_test() -> Client {Client::new()}

    #[rstest]
    #[tokio::test]
    async fn test_make_good_request(client_for_test: Client){
        let result = make_request("http://example.com", &client_for_test).await;
        assert!(result.is_ok());
    }

    #[rstest]
    #[tokio::test]
    async fn test_make_bad_request(client_for_test: Client){
        let result = make_request("http://does-not-exist.com", &client_for_test).await;
        assert!(result.is_err());
    }
}
