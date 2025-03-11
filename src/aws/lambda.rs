//! Client for AWS Lambda.
//!
//! Provides optimized methods for accessing AWS Lambda.
use aws_config::SdkConfig;
use aws_sdk_lambda;

// Maximum results for `ListFunctions` is 50, regardless of a larger configured size.
const PAGINATION_SIZE: i32 = 50;

#[derive(Clone, Debug, Ord, Eq, PartialOrd, PartialEq)]
pub struct Function {
    pub name: String,
}

impl Function {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

/// Client instance for AWS Lambda
pub struct Client {
    client: aws_sdk_lambda::Client,
}

impl Client {
    /// Create a new AWS Lambda client with the provided [`SdkConfig`].
    ///
    /// Using [`config::load_config()`](super::config::load_config()) is recommended to get an
    /// `SdkConfig` instance from the environment.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[tokio::main]
    /// # async fn main() {
    /// use aws_logs_tui::aws::{config, lambda};
    ///
    /// let sdk_config = config::load_config(None, None).await;
    ///
    /// let lambda_client = lambda::Client::new(&sdk_config);
    /// # }
    /// ```
    pub fn new(config: &SdkConfig) -> Self {
        let client = aws_sdk_lambda::Client::new(config);

        Self { client }
    }

    /// Get _all_ AWS Lambda function names, in sorted order.
    ///
    /// The paginated results from AWS Lambda are automatically iterated
    /// to collect all function names as a single, complete list.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[tokio::main]
    /// # async fn main() {
    /// # use aws_logs_tui::aws::{config, lambda};
    /// # let sdk_config = config::load_config(None, None).await;
    /// let lambda_client = lambda::Client::new(&sdk_config);
    ///
    /// let lambda_function_names = lambda_client.get_all_function_names().await;
    /// # }
    /// ```
    pub async fn get_all_functions(&self) -> Vec<Function> {
        let mut function_names = Vec::new();
        let mut next_marker = None;

        loop {
            let mut list_functions_request =
                self.client.list_functions().max_items(PAGINATION_SIZE);
            if let Some(marker) = next_marker {
                list_functions_request = list_functions_request.marker(marker);
            }

            let list_functions_response = list_functions_request
                .send()
                .await
                .expect("Failed to list lambda functions");
            let functions = list_functions_response.functions();
            for function in functions {
                if let Some(name) = &function.function_name {
                    function_names.push(Function::new(&name.clone()))
                }
            }

            next_marker = list_functions_response.next_marker().map(String::from);

            if next_marker.is_none() {
                break;
            }
        }

        function_names.sort();

        function_names
    }
}
