// modual pretaining to formatting data types and raw string data
use reqwest;
/// max size for truncatable values
const MAX_LENGTH: usize = 500;

//local
use crate::errors::{ErrorReason, ValidError};
use crate::types::{self as internalTypes, BuildResult, RequestData};
//alias for any fucntion that builds a custom type
//weird this isnt a funcion that comes with the lib
fn version_to_string(v: reqwest::Version) -> internalTypes::BuildResult<String> {
    Ok(match v {
        reqwest::Version::HTTP_09 => "HTTP/0.9".to_string(),
        reqwest::Version::HTTP_10 => "HTTP/1.0".to_string(),
        reqwest::Version::HTTP_11 => "HTTP/1.1".to_string(),
        reqwest::Version::HTTP_2 => "HTTP/2.0".to_string(),
        reqwest::Version::HTTP_3 => "HTTP/3.0".to_string(),
        _ => {
            return Err(ValidError::build(
                ErrorReason::FormatError,
                "invalid http verison given".to_string(),
            ))
        }
    })
}
//this will be the funcion to take in response and build it to
// the custom type so it can be displayed in a readable fashion
pub fn build_response_data(
    data: reqwest::blocking::Response,
    truncate: bool,
    safe_view: bool,
) -> internalTypes::BuildResult<internalTypes::ReponseData> {
    //match statements are kinda awsesome
    let data = internalTypes::ReponseData {
        status: data.status().to_string(),
        content_length: match data.content_length() {
            Some(c) => c.to_string(),
            //if error then the length of the content must be 0
            None => "0".to_string(),
        },
        headers: if !safe_view {
            crate::formatter::headermap_to_string(data.headers())
        } else {
            "headers hidden".to_string()
        },
        remote_address: match data.remote_addr() {
            Some(a) => a.to_string(),
            None => "failed to find the servers address".to_string(),
        },
        version: crate::formatter::version_to_string(data.version())?,
        body: match data.text() {
            Ok(b) => {
                if truncate && b.len() >= MAX_LENGTH {
                    // added to check for truncation later
                    "~!$body recived but longer than 500 words...".to_string()
                } else {
                    b
                }
            }
            //this error means that the body is empty so i wont touch it
            Err(_) => "no body in response".to_string(),
        },
        truncate,
        safe_view,
    };
    Ok(data)
}

/// will replace the single quotes with double quotes and make the string into a raw stirng
pub fn encode_body(body: String) -> String {
    body.trim().replace("'", r#"""#)
}

pub fn headermap_to_string(header: &reqwest::header::HeaderMap) -> String {
    let mut final_string = "".to_string();
    for val in header.iter() {
        let current = format!(
            "{}:{}",
            val.0.to_string(),
            val.1.to_str().unwrap_or_default().to_string()
        );
        final_string = format!("{}{}|", final_string, current);
    }
    final_string
}

pub fn parse_json_types(head: String) -> Vec<internalTypes::JsonTypes> {
    let mut harray: Vec<internalTypes::JsonTypes> = Vec::new();
    //this is to check all of the default values possible
    if head == "noheaders".to_string()
        || head == "".to_string()
        || head == "headers hidden".to_string()
    {
        return harray;
    }
    let head = head.replace("{", "");
    let head = head.replace("}", "");
    let h: Vec<String> = head.split(",").into_iter().map(|c| c.into()).collect();
    let xx: Vec<_> = h
        .iter()
        .map(move |val| {
            val.split_once(":")
                .expect("invalid json type given to be parsed")
        })
        .collect();
    for val in xx.into_iter() {
        let w = internalTypes::JsonTypes {
            key: std::string::String::from(val.0),
            value: std::string::String::from(val.1),
        };
        harray.push(w);
    }
    harray
}
fn format_regular_parmas(reg_params: String) -> String {
    let mut final_params = "".to_string();
    if reg_params == " ".to_string() || reg_params == "".to_string() {
        return final_params;
    }
    final_params = format!("/{}", reg_params);
    final_params
}
//will create the string what will be added to the final url stirng
//needs to iterate though the vec of jsontypes and the needed symbols for the query params
fn format_query_params(query_params: Vec<internalTypes::JsonTypes>) -> String {
    let mut final_url = "/?".to_string();
    // need this to remove the trailing "/" at the end of the url
    if query_params.len() == 0 {
        return "".to_string();
    }
    //if there is only one query param then format it then return it
    if query_params.len() == 1 {
        final_url = format!(
            "{}{}={}",
            final_url, query_params[0].key, query_params[0].value
        );
        return final_url;
    }
    for param in query_params {
        let curr_string = format!("{}={}&", param.key, param.value);
        final_url = format!("{}{}", final_url, curr_string);
    }
    //doing this slice will remove the trailing "&"
    final_url[..final_url.len() - 1].to_string()
}
pub fn to_header_map(input: Vec<internalTypes::JsonTypes>) -> reqwest::header::HeaderMap {
    let mut map = reqwest::header::HeaderMap::new();
    if input.len() == 0 {
        return map;
    }
    let _ = input.iter().map(|val| {
        map.insert(
            reqwest::header::HeaderName::from_bytes(val.key.as_bytes()).unwrap(),
            val.value.parse().unwrap(),
        )
    });
    map
}
//this will be a function that will add the query params and regular
//params to the url before the request is made
pub fn build_url(
    base_url: String,
    query_params: Vec<internalTypes::JsonTypes>,
    params: String,
) -> String {
    let query_params = crate::formatter::format_query_params(query_params);
    let params = crate::formatter::format_regular_parmas(params);

    format!("{}{}{}", base_url, params, query_params)
}
// TOP LEVEL CALL
pub fn build_request_data(args: internalTypes::ClientArgs) -> BuildResult<RequestData> {
    let data = internalTypes::RequestData {
        body: match args.body {
            Some(b) => crate::formatter::encode_body(b),
            None => "".to_string(),
        },
        url: match args.url {
            //only add the https to the front of domains not full urls
            Some(u) => u,
            None => match args.domain {
                Some(d) => d,
                None => {
                    return Err(ValidError::build(
                        ErrorReason::FormatError,
                        "neither full url or domain name was passed".to_string(),
                    ))
                }
            },
        },
        xheaders: match args.xheaders {
            Some(x) => {
                if !args.safe_mode {
                    x
                } else {
                    "headers hidden".to_string()
                }
            }
            None => "noheaders".to_string(),
        },
        params: match args.params {
            Some(p) => p,
            None => "".to_string(),
        },
        querys: match args.querys {
            Some(q) => q,
            None => "".to_string(),
        },
        rtype: match args.rtype {
            Some(t) => internalTypes::RequestType::string_to_val(t.as_str())?,
            None => internalTypes::RequestType::GET,
        },
        truncate: args.truncate,
        safe_mode: args.safe_mode,
    };
    Ok(data)
}
