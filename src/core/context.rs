use std::marker::PhantomData;

use eyre::{bail, Result};
use inquire::{
    validator::{StringValidator, Validation, ValueRequiredValidator},
    CustomUserError, Text,
};
use url::Url;

use super::{cli::Cli, nocodb::NocoDB};

/// Phantom data to represent the status of the context.
pub trait ContextStatus {}
/// Context status when it is uninitialized.
pub struct Uninitialized;
/// Context status when it is initialized.
pub struct Initialized;
impl ContextStatus for Uninitialized {}
impl ContextStatus for Initialized {}

#[derive(Debug, Clone)]
/// Context object to hold the CLI arguments and the NocoDB client.
pub struct Context<S: ContextStatus> {
    pub nocodb: NocoDB,
    pub args: Cli,
    pub status: PhantomData<S>,
}

impl Context<Uninitialized> {
    /// Create a new context from the CLI arguments.
    ///
    /// If the NocoDB base URL and API token are provided, they will be used to create a new NocoDB client.
    ///
    /// This should be called to create a new context with the uninitialized status.
    pub fn from_args(args: &Cli) -> Self {
        let nocodb: NocoDB;

        if args.nocodb_base_url.is_some() && args.nocodb_api_token.is_some() {
            nocodb = NocoDB::new(
                args.nocodb_base_url.clone().unwrap(),
                args.nocodb_api_token.clone().unwrap(),
            );
        } else {
            nocodb = NocoDB::default();
        }

        Self {
            nocodb,
            args: args.clone(),
            status: PhantomData,
        }
    }

    /// Initialize the context.
    ///
    /// This should be called once all the required arguments are provided.
    pub fn init(&self) -> Result<Context<Initialized>> {
        if true == self.has_missing_args() {
            bail!("Missing arguments");
        }

        Ok(Context {
            nocodb: NocoDB::new(
                self.args.nocodb_base_url.clone().unwrap(),
                self.args.nocodb_api_token.clone().unwrap(),
            ),
            args: self.args.clone(),
            status: PhantomData,
        })
    }

    /// Check if there are any missing arguments to properly run the command.
    pub fn has_missing_args(&self) -> bool {
        self.args.nocodb_base_url.is_none()
            || self.args.nocodb_api_token.is_none()
            || self.args.nocodb_table_id.is_none()
    }

    /// Collect missing arguments from the user.
    pub fn collect_missing_args(&mut self) -> Result<()> {
        if self.args.nocodb_base_url.is_none() {
            let base_url = Text::new("Enter NocoDB base URL: ")
                .with_help_message("You can find the NocoDB base URL in your browser address bar")
                .with_validator(ValueRequiredValidator::default())
                .with_validator(UrlValidator::default())
                .prompt()?;

            self.args.nocodb_base_url = Some(base_url);
        }

        if self.args.nocodb_api_token.is_none() {
            let api_token = Text::new("Enter NocoDB API token: ")
                .with_help_message(
                    "You can find the NocoDB API token in the settings menu, under the API section",
                )
                .with_validator(ValueRequiredValidator::default())
                .prompt()?;

            self.args.nocodb_api_token = Some(api_token);
        }

        if self.args.nocodb_table_id.is_none() {
            let table_id: String = Text::new("Enter NocoDB Table ID: ")
                .with_help_message(
                    "You can find the table ID by clicking on the three dots near the table name",
                )
                .with_validator(ValueRequiredValidator::default())
                .prompt()?;

            self.args.nocodb_table_id = Some(table_id);
        }

        Ok(())
    }
}

// -- utils --

#[derive(Clone)]
/// Validator to check if a string is a valid URL.
pub struct UrlValidator {
    message: String,
}

impl UrlValidator {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Default for UrlValidator {
    fn default() -> Self {
        Self {
            message: "Invalid URL format".to_owned(),
        }
    }
}

impl StringValidator for UrlValidator {
    fn validate(&self, input: &str) -> Result<Validation, CustomUserError> {
        Ok(if Url::parse(input).is_err() {
            Validation::Invalid(self.message.as_str().into())
        } else {
            Validation::Valid
        })
    }
}
