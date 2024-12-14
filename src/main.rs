use tokio;
use reqwest;
use async_trait::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

use std::fmt;


// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
struct NetworkError;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl From<reqwest::Error> for NetworkError {
    fn from(_t: reqwest::Error) -> NetworkError {
        NetworkError
    }
}

// Order of macros matters. See https://medium.com/vortechsa/mocking-in-async-rust-248b012c5e99
#[cfg_attr(test, automock)]
#[async_trait]
trait HttpBodyGetter {  // Need to mock this trait for unit tests
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
    let result = make_request("http://example.com", &client).await?;
    println!("Response: {}", result);
    Ok(())
}

async fn make_request(url: &str, client: &impl HttpBodyGetter) -> Result<String, NetworkError> {
    let body = client.get_http_response_body(url).await?;
    Ok(body)
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
    async fn test_make_good_request(client_for_test: impl HttpBodyGetter){

        //client_for_test.expect_get_http_response_body().times(1);

        let result = client_for_test.get_http_response_body("http://example.com").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(),"<!doctype html></html>");
    }

    #[rstest]
    #[tokio::test]
    async fn test_make_bad_request(client_for_test: impl HttpBodyGetter){
        let result = make_request("http://does-not-exist.com", &client_for_test).await;
        assert!(result.is_err());
    }
}
