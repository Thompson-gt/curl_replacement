//this module will be the handlers of the project i.e. making the network requests or running the program

// maybe change this to request pool or non blocking
use clap::Parser;
use reqwest::blocking::RequestBuilder;

use crate::displayer;
use crate::errors::ValidError;
use crate::formatter;
use crate::types as interalTypes;

pub fn make_request(request_data: interalTypes::RequestData) -> interalTypes::BuildResult<()> {
    //cloning of 2 bools are cheap enough for me not to bother with other ways around the borrow checker
    let truncate = request_data.truncate.clone();
    let safe_mode = request_data.safe_mode.clone();
    displayer::display_request_data(&request_data);
    let http_client = reqwest::blocking::Client::new();
    let res = match crate::handler::configure_request(request_data, http_client).send() {
        Ok(r) => r,
        Err(e) => {
            return Err(ValidError::build(
                crate::errors::ErrorReason::NetworkError,
                e.to_string(),
            ))
        }
    };

    let response_data = formatter::build_response_data(res, truncate, safe_mode)?;
    displayer::display_response_data(&response_data);
    Ok(())
}

fn configure_request(
    request_data: interalTypes::RequestData,
    http_client: reqwest::blocking::Client,
) -> RequestBuilder {
    //build the url here before returing the request to be made
    let current_headers: Vec<interalTypes::JsonTypes> =
        formatter::parse_json_types(request_data.xheaders);
    let current_headers = formatter::to_header_map(current_headers);
    let qp = formatter::parse_json_types(request_data.querys);
    let final_url = formatter::build_url(request_data.url, qp, request_data.params);
    match request_data.rtype {
        interalTypes::RequestType::GET => http_client
            .get(final_url)
            .headers(current_headers)
            .body(request_data.body),
        interalTypes::RequestType::POST => http_client
            .post(final_url)
            .headers(current_headers)
            .body(request_data.body),
        interalTypes::RequestType::PUT => http_client
            .put(final_url)
            .headers(current_headers)
            .body(request_data.body),
        interalTypes::RequestType::DELETE => http_client
            .delete(final_url)
            .headers(current_headers)
            .body(request_data.body),
    }
}
// main entry point to the program 
pub fn run_program() {
    match std::env::args().len() {
        1 => crate::displayer::display_help(),
        _ => {
            let args = interalTypes::ClientArgs::parse();
            let request_data = match formatter::build_request_data(args) {
                Ok(data) => data,
                Err(e) => return displayer::display_final_message_failed(e),
            };

            match make_request(request_data) {
                Ok(_) => displayer::display_final_message_success(),
                Err(e) => displayer::display_final_message_failed(e),
            }
        }
    }
}
