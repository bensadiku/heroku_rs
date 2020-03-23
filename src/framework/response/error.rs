use std::error::Error;
use std::fmt;
use std::fmt::Debug;

/// HerokuApiError
/// 
/// Failing responses will have an appropriate status and a JSON body containing more details about a particular error. See error responses for more example ids.
/// 
/// [See the Heroku docs for this error struct](https://devcenter.heroku.com/articles/platform-api-reference#error-attributes)
#[derive(Deserialize, Debug, Default)]
pub struct HerokuApiError {
    /// end user message of error raised
    pub message: String,
    /// id of error raised
    pub id: String,
}

/// An enum to classify which errors are what.
#[derive(Debug)]
pub enum HerokuApiFailure {
    /// If Heroku API returned a Error code, this enum is used to handle the error
    Error(reqwest::StatusCode, HerokuApiError),
    /// If there was a invalid response, or the response failed, this enum is used to handle the error
    Invalid(reqwest::Error),
}

impl PartialEq for HerokuApiError {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.message == other.message
    }
}

impl Error for HerokuApiFailure {}
impl Error for HerokuApiError {}

impl Eq for HerokuApiFailure {}
impl Eq for HerokuApiError {}

impl PartialEq for HerokuApiFailure {
    fn eq(&self, other: &HerokuApiFailure) -> bool {
        match (self, other) {
            (HerokuApiFailure::Invalid(e1), HerokuApiFailure::Invalid(e2)) => {
                e1.to_string() == e2.to_string()
            }
            (HerokuApiFailure::Error(status1, e1), HerokuApiFailure::Error(status2, e2)) => {
                status1 == status2 && e1 == e2
            }
            _ => false,
        }
    }
}

impl fmt::Display for HerokuApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {} {} ", self.id, self.message)
    }
}

impl From<reqwest::Error> for HerokuApiFailure {
    fn from(error: reqwest::Error) -> Self {
        HerokuApiFailure::Invalid(error)
    }
}

impl fmt::Display for HerokuApiFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HerokuApiFailure::Error(status, error) => {
                let mut output = "".to_owned();
                output.push_str(&format!("HTTP: {}", status));

                output.push_str(&format!("\n{} {} ", error.id, error.message));
                write!(f, "{}", output)
            }
            HerokuApiFailure::Invalid(err) => write!(f, "{}", err),
        }
    }
}
