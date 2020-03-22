use std::error::Error;
use std::fmt;
use std::fmt::Debug;

#[derive(Deserialize, Debug, Default)]
pub struct HerokuApiError {
    pub message: String,
    pub id: String,
}

#[derive(Debug)]
pub enum HerokuApiFailure {
    Error(reqwest::StatusCode, HerokuApiError),
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
