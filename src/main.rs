use tokio;
use reqwest::{Client,Error};
use async_trait::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

// Order of macros matters. See https://medium.com/vortechsa/mocking-in-async-rust-248b012c5e99
#[cfg_attr(test, automock)]
#[async_trait]
trait HttpBodyGetter {  // Need to mock this trait for unit tests
    async fn get_http_response_body(&self, url: &str) -> Result<String, Error>;
}

#[async_trait]
impl HttpBodyGetter for Client {
    async fn get_http_response_body(&self, url: &str) -> Result<String, Error> {
        self.get(url)
            .send().await?
            .error_for_status()?
            .text().await
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();
    let result = make_request("http://example.com", &client).await?;
    println!("Response: {}", result);
    Ok(())
}

async fn make_request(url: &str, client: &impl HttpBodyGetter) -> Result<String, Error> {
    let body = client.get_http_response_body(url).await?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{rstest,fixture};

    #[fixture]
    pub fn client_for_test() -> Client {
        //let mock_client = MockHttpGetter();
        let mock_client = Client::new();
        return mock_client;
        // Make this fixture return one mock client with the correct logic for 
        // Make it mutable so that tests can say what they expect from it
    }

    #[rstest]
    #[tokio::test]
    async fn test_make_good_request(/*client_for_test: Client*/){
        let mut mock_client = MockHttpBodyGetter::new();
        mock_client.expect_get_http_response_body()
            .times(1)
            .returning(|_| Ok("<!doctype html></html>".to_string()));
        assert_eq!(mock_client.get_http_response_body("argument").await.unwrap(),"<!doctype html></html>");
        //let result = make_request("http://example.com", &mock_client).await;
        //assert!(result.is_ok());
    }

    #[rstest]
    #[tokio::test]
    async fn test_make_bad_request(/*client_for_test: Client*/){
        let client_for_test = Client::new();
        /*
        let mock_client = MockHttpBodyGetter::new()
            .expect_get_http_response_body()
            .returning(|_| Error::new( reqwest::error::Kind, )); */
        let result = make_request("http://does-not-exist.com", &client_for_test).await;
        assert!(result.is_err());
    }
}
