//! Customize `aws_config` with optional Profile and Region values.
//!
//! If Profile or Region are provided, use those to override the values
//! inferred from the environment or AWS configuration.
//!
//! Uses the latest AWS SDK behavior.
use aws_config::{BehaviorVersion, Region, SdkConfig};

/// Load the AWS configuration for use with AWS clients.
///
/// Apply the optional `profile` and `region` values to override
/// the inferred defaults from the environment or AWS configuration.
///
/// # Examples
///
/// With optional values.
///
/// ```
/// # #[tokio::main]
/// # async fn main() {
/// use aws_logs_tui::aws::config;
///
/// let sdk_config = config::load_config(
///     Some(String::from("my-aws-profile")),
///     Some(String::from("us-west-2")))
///     .await;
/// # }
/// ```
///
/// Without optional values.
///
/// ```
/// # #[tokio::main]
/// # async fn main() {
/// use aws_logs_tui::aws::config;
///
/// let sdk_config = config::load_config(None, None).await;
/// # }
/// ```
pub async fn load_config(profile: Option<String>, region: Option<String>) -> SdkConfig {
    let mut config = aws_config::defaults(BehaviorVersion::latest());
    if let Some(profile_name) = profile {
        config = config.profile_name(profile_name);
    }
    if let Some(region) = region {
        config = config.region(Region::new(region.clone()));
    }

    config.load().await
}
