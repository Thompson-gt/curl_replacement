// this will be the package that will handle the building of the request and possiblly making the request
// will add all of the promting things in this file if i end up doing it

pub mod handler {

    use clap::{self, Parser};
    use reqwest::blocking::RequestBuilder;

    use crate::displayers;
    use crate::formatters;

    pub fn make_request(request_data: formatters::format::RequestData) {
        let truncate = request_data.truncate.clone();
        //display request data
        displayers::display::display_request_data(&request_data);
        //make the client in here becuase i am assuming its a synchronious req
        let http_client = reqwest::blocking::Client::new();
        //configure the request and make it (handle the error)
        let res =
            match crate::handlers::handler::configure_request(request_data, http_client).send() {
                Ok(r) => r,
                Err(e) => panic!("error when making the request:{}", e),
            };
        //build response data
        let response_data = formatters::format::build_response_data(res, truncate);
        //display response data
        displayers::display::display_response_data(&response_data);
    }

    fn configure_request(
        request_data: formatters::format::RequestData,
        http_client: reqwest::blocking::Client,
    ) -> RequestBuilder {
        //build the url here before returing the request to be made
        let current_headers: Vec<formatters::format::JsonTypes> =
            formatters::format::parse_json_types(request_data.xheaders);
        let current_headers = formatters::format::to_header_map(current_headers);
        let qp = formatters::format::parse_json_types(request_data.querys);
        let final_url = formatters::format::build_url(request_data.url, qp, request_data.params);
        match request_data.rtype {
            formatters::format::RequestType::GET => http_client
                .get(final_url)
                .headers(current_headers)
                .body(request_data.body),
            formatters::format::RequestType::POST => http_client
                .post(final_url)
                .headers(current_headers)
                .body(request_data.body),
            formatters::format::RequestType::PUT => http_client
                .put(final_url)
                .headers(current_headers)
                .body(request_data.body),
            formatters::format::RequestType::DELETE => http_client
                .delete(final_url)
                .headers(current_headers)
                .body(request_data.body),
        }
    }
    //this should just display the help message when no arguments are given to
    //the program
    pub fn run_program() {
        match std::env::args().len() {
            1 => crate::displayers::display::display_help(),
            _ => {
                let args = formatters::format::ClientArgs::parse();
                let request_data = formatters::format::build_request_data(args);
                make_request(request_data);
            }
        }
    }
}
