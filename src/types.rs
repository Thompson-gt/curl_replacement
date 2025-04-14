// this needs to be the file that will hold all the types so I dont have to search for them

//HTTPS TYPES START

use crate::errors::ValidError;

/// alais for the result for the construction of internal types
pub type BuildResult<T> = std::result::Result<T, ValidError>;

#[derive(Debug)]
pub struct ReponseData {
    pub status: String,
    pub content_length: String,
    pub headers: String,
    pub remote_address: String,
    pub version: String,
    pub body: String,
    pub truncate: bool,
    pub safe_view: bool,
}

#[derive(Debug, Clone)]
pub struct RequestData {
    pub body: String,
    pub url: String,
    pub xheaders: String,
    pub params: String,
    pub querys: String,
    pub rtype: RequestType,
    pub truncate: bool,
    pub safe_mode: bool,
}

#[derive(Debug, Clone)]
pub enum RequestType {
    GET,
    DELETE,
    POST,
    PUT,
}
impl RequestType {
    pub fn string_to_val(input: &str) -> Result<RequestType, ValidError> {
        match input {
            "get" => Ok(RequestType::GET),
            "delete" => Ok(RequestType::DELETE),
            "post" => Ok(RequestType::POST),
            "put" => Ok(RequestType::PUT),
            _ => Err(ValidError::build(
                crate::errors::ErrorReason::FormatError,
                "invalid request type".to_string(),
            )),
        }
    }
    pub fn request_to_string(input: &RequestType) -> String {
        match input {
            RequestType::GET => "GET".to_string(),
            RequestType::DELETE => "DELETE".to_string(),
            RequestType::POST => "POST".to_string(),
            RequestType::PUT => "PUT".to_string(),
        }
    }
}

//HTTP TYPES END

use clap::Parser;
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct ClientArgs {
    /// the body for the request you wish to make(needs to be in json format)
    #[arg(short, long)]
    pub body: Option<String>,
    /// the full url including http or https(need to use this to specify https instead of http)
    #[arg(long)]
    pub url: Option<String>,
    ///The base url for the request to be built with (defaults to http)
    #[arg(short, long)]
    pub domain: Option<String>,
    ///The https headers to be included with the request format == {key:val,...}
    #[arg(short, long)]
    pub xheaders: Option<String>,
    /// Url params to be added into the string when request is made format == {key:val,...}
    #[arg(short, long)]
    pub params: Option<String>,
    /// Url query params to be added into the string when request is made
    #[arg(short, long)]
    pub querys: Option<String>,
    /// HTTP request method
    #[arg(short, long)]
    pub rtype: Option<String>,
    /// will hide the body response if greater than 500 words(to use this pass the flag no value needed)
    #[arg(short, long)]
    #[clap(default_value_t = false)]
    pub truncate: bool,
    /// will hide the headers in the request and response when displayed to protect sensitive data
    #[arg(short, long)]
    #[clap(default_value_t = false)]
    pub safe_mode: bool,
}

/// key value pair for readability
#[derive(Debug, Clone)]
pub struct JsonTypes {
    pub key: String,
    pub value: String,
}
