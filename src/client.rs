// Tokio/Future Imports
use futures::future::ok;
use futures::{Future, Stream};
use tokio_core::reactor::Core;

// Hyper Imports
use hyper::header::{HeaderName, HeaderValue, IF_NONE_MATCH};
use hyper::StatusCode;
use hyper::{self, Body, HeaderMap};
use hyper::{Client, Request};
#[cfg(feature = "rustls")]
type HttpsConnector = hyper_rustls::HttpsConnector<hyper::client::HttpConnector>;
#[cfg(feature = "rust-native-tls")]
use hyper_tls;
#[cfg(feature = "rust-native-tls")]
type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

// Serde Imports
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

// Internal Library Imports
use crate::errors::*;
use crate::teams;
use crate::apps;
use crate::account;
use crate::utils::url_join;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Heroku {
    token: String,
    core: Rc<RefCell<Core>>,
    client: Rc<Client<HttpsConnector>>,
}

impl Clone for Heroku {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            core: Rc::clone(&self.core),
            client: Rc::clone(&self.client),
        }
    }
}

new_type!(GetQueryBuilder);

new_type!(PutQueryBuilder);

new_type!(PostQueryBuilder);

new_type!(DeleteQueryBuilder);

new_type!(PatchQueryBuilder);

new_type!(CustomQuery);

exec!(CustomQuery);

pub trait Executor {
    fn execute<T>(self) -> Result<(HeaderMap, StatusCode, Option<T>)>
    where
        T: DeserializeOwned;
}

impl Heroku {
    pub fn new<T>(token: T) -> Result<Self>
    where
        T: ToString,
    {
        let core = Core::new()?;
        #[cfg(feature = "rustls")]
        let client = Client::builder().build(HttpsConnector::new(4));
        #[cfg(feature = "rust-native-tls")]
        let client = Client::builder().build(HttpsConnector::new(4)?);
        Ok(Self {
            token: token.to_string(),
            core: Rc::new(RefCell::new(core)),
            client: Rc::new(client),
        })
    }

    /// Get the currently set Authorization Token
    pub fn get_token(&self) -> &str {
        &self.token
    }

    /// Change the currently set Authorization Token using a type that can turn
    /// into an &str. Must be a valid API Token for requests to work.
    pub fn set_token<T>(&mut self, token: T)
    where
        T: ToString,
    {
        self.token = token.to_string();
    }

    /// Begin building up a GET request to Heroku
    pub fn get(&self) -> GetQueryBuilder {
        self.into()
    }

    /// Begin building up a PUT request with no data to Heroku
    pub fn put_empty(&self) -> PutQueryBuilder {
        self.into()
    }

    /// Begin building up a POST request with no data to Heroku
    pub fn post_empty(&self) -> PostQueryBuilder {
        self.into()
    }

    /// Begin building up a PUT request with data to Heroku
    pub fn put<T>(&self, body: T) -> PutQueryBuilder
    where
        T: Serialize,
    {
        let mut qb: PutQueryBuilder = self.into();
        if let Ok(mut qbr) = qb.request {
            let serialized = serde_json::to_vec(&body);
            match serialized {
                Ok(json) => {
                    *qbr.get_mut().body_mut() = json.into();
                    qb.request = Ok(qbr);
                }
                Err(_) => {
                    qb.request = Err("Unable to serialize data to JSON".into());
                }
            }
        }
        qb
    }

    /// Begin building up a POST request with data to Heroku
    pub fn post<T>(&self, body: T) -> PostQueryBuilder
    where
        T: Serialize,
    {
        let mut qb: PostQueryBuilder = self.into();
        if let Ok(mut qbr) = qb.request {
            let serialized = serde_json::to_vec(&body);
            match serialized {
                Ok(json) => {
                    *qbr.get_mut().body_mut() = json.into();
                    qb.request = Ok(qbr);
                }
                Err(_) => {
                    qb.request = Err("Unable to serialize data to JSON".into());
                }
            }
        }

        qb
    }

    /// Begin building up a PATCH request with data to Heroku
    pub fn patch<T>(&self, body: T) -> PatchQueryBuilder
    where
        T: Serialize,
    {
        let mut qb: PatchQueryBuilder = self.into();
        if let Ok(mut qbr) = qb.request {
            let serialized = serde_json::to_vec(&body);
            match serialized {
                Ok(json) => {
                    *qbr.get_mut().body_mut() = json.into();
                    qb.request = Ok(qbr);
                }
                Err(_) => {
                    qb.request = Err("Unable to serialize data to JSON".into());
                }
            }
        }
        qb
    }

    /// Begin building up a DELETE request with data to Heroku
    pub fn delete<T>(&self, body: T) -> DeleteQueryBuilder
    where
        T: Serialize,
    {
        let mut qb: DeleteQueryBuilder = self.into();

        if let Ok(mut qbr) = qb.request {
            let serialized = serde_json::to_vec(&body);
            match serialized {
                Ok(json) => {
                    *qbr.get_mut().body_mut() = json.into();
                    qb.request = Ok(qbr);
                }
                Err(_) => {
                    qb.request = Err("Unable to serialize data to JSON".into());
                }
            }
        }
        qb
    }

    /// Begin building up a DELETE request without data to Heroku
    pub fn delete_empty(&self) -> DeleteQueryBuilder {
        self.into()
    }
}
impl<'g> GetQueryBuilder<'g> {
    /// Pass in an endpoint not covered by the API in the form of the following:
    /// It can be whatever endpoint or url string that's needed. This will allow
    /// you to get functionality out of the library as items are still added or
    /// if you need access to a hidden endpoint.
    func_client!(custom_endpoint, CustomQuery, endpoint_str);

    /// Query the Teams endpoint
    func_client!(teams, teams::get::Teams<'g>);

    /// Query the App endpoint
    func_client!(apps, apps::get::Apps<'g>);

    /// Query the Account endpoint
    func_client!(account, account::get::Account<'g>);
    
    /// Query the Users account endpoint
    func_client!(accounts, account::get::Accounts<'g>);

    /// Add an etag to the headers of the request
    pub fn set_etag(mut self, tag: impl Into<HeaderValue>) -> Self {
        match self.request {
            Ok(mut req) => {
                req.get_mut()
                    .headers_mut()
                    .insert(IF_NONE_MATCH, tag.into());
                self.request = Ok(req);
                self
            }
            Err(_) => self,
        }
    }
}

impl<'g> PutQueryBuilder<'g> {
    /// Pass in an endpoint not covered by the API in the form of the following:
    /// It can be whatever endpoint or url string that's needed. This will allow
    /// you to get functionality out of the library as items are still added or
    /// if you need access to a hidden endpoint.
    func_client!(custom_endpoint, CustomQuery, endpoint_str);
    
    func_client!(apps, apps::put::Apps<'g>);

    func_client!(teams, teams::put::Teams<'g>);

    func_client!(account, account::put::Account<'g>);

    /// Add an etag to the headers of the request
    pub fn set_etag(mut self, tag: impl Into<HeaderValue>) -> Self {
        match self.request {
            Ok(mut req) => {
                req.get_mut()
                    .headers_mut()
                    .insert(IF_NONE_MATCH, tag.into());
                self.request = Ok(req);
                self
            }
            Err(_) => self,
        }
    }
}

impl<'g> DeleteQueryBuilder<'g> {
    /// Pass in an endpoint not covered by the API in the form of the following:
    /// It can be whatever endpoint or url string that's needed. This will allow
    /// you to get functionality out of the library as items are still added or
    /// if you need access to a hidden endpoint.
    func_client!(custom_endpoint, CustomQuery, endpoint_str);
   
    func_client!(apps, apps::delete::Apps<'g>);

    func_client!(teams, teams::delete::Teams<'g>);

    func_client!(account, account::delete::Account<'g>);

    func_client!(accounts, account::delete::Accounts<'g>);

    /// Add an etag to the headers of the request
    pub fn set_etag(mut self, tag: impl Into<HeaderValue>) -> Self {
        match self.request {
            Ok(mut req) => {
                req.get_mut()
                    .headers_mut()
                    .insert(IF_NONE_MATCH, tag.into());
                self.request = Ok(req);
                self
            }
            Err(_) => self,
        }
    }
}

impl<'g> PostQueryBuilder<'g> {
    /// Pass in an endpoint not covered by the API in the form of the following:
    /// It can be whatever endpoint or url string that's needed. This will allow
    /// you to get functionality out of the library as items are still added or
    /// if you need access to a hidden endpoint.
    func_client!(custom_endpoint, CustomQuery, endpoint_str);

    func_client!(apps, apps::post::Apps<'g>);

    func_client!(teams, teams::post::Teams<'g>);

    func_client!(account, account::post::Account<'g>);

    func_client!(accounts, account::post::Accounts<'g>);

    /// Add an etag to the headers of the request
    pub fn set_etag(mut self, tag: impl Into<HeaderValue>) -> Self {
        match self.request {
            Ok(mut req) => {
                req.get_mut()
                    .headers_mut()
                    .insert(IF_NONE_MATCH, tag.into());
                self.request = Ok(req);
                self
            }
            Err(_) => self,
        }
    }
}

impl<'g> PatchQueryBuilder<'g> {
    /// Pass in an endpoint not covered by the API in the form of the following:
    /// It can be whatever endpoint or url string that's needed. This will allow
    /// you to get functionality out of the library as items are still added or
    /// if you need access to a hidden endpoint.
    func_client!(custom_endpoint, CustomQuery, endpoint_str);

    func_client!(apps, apps::patch::Apps<'g>);

    func_client!(teams, teams::patch::Teams<'g>);

    func_client!(account, account::patch::Account<'g>);

    func_client!(accounts, account::patch::Accounts<'g>);

    /// Add an etag to the headers of the request
    pub fn set_etag(mut self, tag: impl Into<HeaderValue>) -> Self {
        match self.request {
            Ok(mut req) => {
                req.get_mut()
                    .headers_mut()
                    .insert(IF_NONE_MATCH, tag.into());
                self.request = Ok(req);
                self
            }
            Err(_) => self,
        }
    }
}

// From derivations of Heroku to the given type using a certain
// request method
from!(
    @GetQueryBuilder
        => "GET"
    @PutQueryBuilder
        => "PUT"
    @PostQueryBuilder
        => "POST"
    @PatchQueryBuilder
        => "PATCH"
    @DeleteQueryBuilder
        => "DELETE"
);

// Custom Url based from impls
from!(
    @GetQueryBuilder
       => CustomQuery
    @PutQueryBuilder
       => CustomQuery
    @PostQueryBuilder
       => CustomQuery
    @PatchQueryBuilder
       => CustomQuery
    @DeleteQueryBuilder
       => CustomQuery
);

impl<'a> CustomQuery<'a> {
    /// Set custom header for request.
    /// Useful for custom headers (sometimes using in api preview).
    pub fn set_header(
        mut self,
        header_name: impl Into<HeaderName>,
        accept_header: impl Into<HeaderValue>,
    ) -> Self {
        match self.request {
            Ok(mut req) => {
                req.get_mut()
                    .headers_mut()
                    .insert(header_name.into(), accept_header.into());
                self.request = Ok(req);
                self
            }
            Err(_) => self,
        }
    }
}