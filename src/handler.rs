//this module will be the handlers of the project i.e. making the network requests or running the program


    use clap::{self, Parser};
    use reqwest::blocking::RequestBuilder;

    use crate::formatter;
    use crate::displayer;

    type RequestResult = std::result::Result<(), reqwest::Error>;

    pub fn make_request(request_data: formatter::RequestData) -> RequestResult {
        //cloning of 2 bools are cheap enough for me not to bother with other ways around the borrow checker
        let truncate = request_data.truncate.clone();
        let safe_mode = request_data.safe_mode.clone();
        //display request data
        displayer::display::display_request_data(&request_data);
        //make the client in here becuase i am assuming its a synchronious req
        let http_client = reqwest::blocking::Client::new();
        //configure the request and make it (handle the error)
        let res = crate::handler::configure_request(request_data, http_client).send()?;
        //build response data
        let response_data = match formatter::build_response_data(res, truncate, safe_mode)
        {
            Ok(r) => r,
            Err(e) => panic!("error when building response data: {}", e),
        };
        //display response data
        displayer::display::display_response_data(&response_data);
        Ok(())
    }

    fn configure_request(
        request_data: formatter::RequestData,
        http_client: reqwest::blocking::Client,
    ) -> RequestBuilder {
        //build the url here before returing the request to be made
        let current_headers: Vec<formatter::JsonTypes> =
            formatter::parse_json_types(request_data.xheaders);
        let current_headers = formatter::to_header_map(current_headers);
        let qp = formatter::parse_json_types(request_data.querys);
        let final_url = formatter::build_url(request_data.url, qp, request_data.params);
        match request_data.rtype {
            formatter::RequestType::GET => http_client
                .get(final_url)
                .headers(current_headers)
                .body(request_data.body),
            formatter::RequestType::POST => http_client
                .post(final_url)
                .headers(current_headers)
                .body(request_data.body),
            formatter::RequestType::PUT => http_client
                .put(final_url)
                .headers(current_headers)
                .body(request_data.body),
            formatter::RequestType::DELETE => http_client
                .delete(final_url)
                .headers(current_headers)
                .body(request_data.body),
        }
    }
    //made a run function to make a clean way of handling empty arguments to the program
    pub fn run_program() {
        match std::env::args().len() {
            1 => crate::displayer::display::display_help(),
            _ => {
                let args = formatter::ClientArgs::parse();
                let request_data = match formatter::build_request_data(args) {
                    Ok(data) => data,
                    Err(e) => panic!("problem when building the request data: {}", e),
                };
                // let t = request_data.clone();
                // print!("{:?}", t);
                match make_request(request_data) {
                    Ok(_) => displayer::display::display_final_message_success(),
                    Err(e) => displayer::display::display_final_message_failed(e),
                };
            }
        }
    }
