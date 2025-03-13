//this module will handle any kind of output to the terminal

pub mod display {

    //downloaded crates
    use colored::*;
    use std::{env::*, path::Path};

    //local crates
    use crate::formatter;

    fn filled_string(msg: &str) -> ColoredString {
        msg.green().bold().italic()
    }
    fn not_filled_string(msg: &str) -> ColoredString {
        msg.red().bold().strikethrough()
    }
    fn title_string(msg: &str) -> ColoredString {
        msg.blue().bold().italic()
    }
    fn cat_string(msg: &str) -> ColoredString {
        msg.to_string().underline().bold().cyan()
    }
    fn warn_filled(msg: &str) -> ColoredString {
        msg.yellow().bold().italic()
    }
    //this is the funciton to display the request data is a colorful and readable way
    pub fn display_request_data(data: &formatter::RequestData) {
        let welcome_message =
             title_string("\n\n----GIVEN REQUEST----\n\n");
        let body = if data.body == "".to_string() {
            not_filled_string("no body")
        } else {
            filled_string(&data.body)
        };
        let url = filled_string(&data.url);

        let xhead = if data.xheaders == "noheaders".to_string() {
            not_filled_string("no headers")
        } else if data.xheaders == "headers hidden".to_string() {
            warn_filled("headers hidden")
        } else {
            filled_string(&data.xheaders)
        };
        let paras = if data.params == "".to_string() {
            not_filled_string("no params")
        } else {
            filled_string(&data.params)
        };
        let q = if data.querys == "".to_string() {
            not_filled_string("no query params")
        } else {
            filled_string(&data.querys)
        };
        let t = filled_string(
            crate::formatter::RequestType::request_to_string(&data.rtype).as_str(),
        );
        let safe_mode = if data.safe_mode {
            warn_filled("TRUE")
        } else {
            warn_filled("FALSE")
        };
        print!("{}", welcome_message);
        print!(
            "{}: {}\n",
            cat_string("REQUEST URL"),
            url
        );
        print!(
            "{}: {}\n",
            cat_string("REQUEST BODY"),
            body
        );
        print!(
            "{}: {}\n",
            cat_string("REQUEST HEADERS"),
            xhead
        );
        print!(
            "{}: {}\n",
            cat_string("REQUEST PARAMS"),
            paras
        );
        print!(
            "{}: {}\n",
            cat_string("REQUEST QUERY PARAMS"),
            q
        );
        print!(
            "{}: {}\n",
            cat_string("REQUEST TYPE"),
            t
        );
        print!(
            "{}: {}\n",
            cat_string("SAFE MODE?"),
            safe_mode
        );
    }
    pub fn display_response_data(data: &formatter::ReponseData) {
        let welcome_message =
            title_string("\n\n----RECIVED RESPONSE ----\n\n");
        let status: ColoredString;
        if data.status.starts_with("3") {
            status = warn_filled(&data.status);
        } else if data.status.starts_with("4") || data.status.starts_with("5") {
            status = not_filled_string(&data.status);
        } else {
            status = filled_string(&data.status);
        }
        let clen = if data.content_length == "0" {
            not_filled_string(&data.content_length)
        } else {
            filled_string(&data.content_length)
        };
        let headers = if !data.safe_view {
            filled_string(&data.headers)
        } else {
            warn_filled(&data.headers)
        };
        let rm = if data.remote_address.starts_with("failed") {
            not_filled_string(&data.remote_address)
        } else {
            filled_string(&data.remote_address)
        };
        let version = if data.version.starts_with("failed") {
            not_filled_string(&data.version)
        } else {
            filled_string(&data.version)
        };
        let body = if data.body.starts_with("no") {
            not_filled_string(&data.body)
        } else if data.body == "body recived but longer than 500 words...".to_string() {
            warn_filled("body recived but longer than 500 words...")
        } else {
            filled_string(&data.body)
        };
        let truncate = if data.truncate {
            filled_string("YES")
        } else {
            warn_filled("NO")
        };
        let safe_mode = if data.safe_view {
            warn_filled("TRUE")
        } else {
            warn_filled("FALSE")
        };
        print!("{}", welcome_message);
        print!(
            "{}: {}\n",
            cat_string("REQUEST STATUS"),
            status
        );
        print!(
            "{}: {}\n",
            cat_string("CONTENT_LENGTH"),
            clen
        );
        print!(
            "{}: {}\n",
            cat_string("REQUEST HEADERS"),
            headers
        );
        print!(
            "{}: {}\n",
            cat_string("REQUEST REMOTE ADDRESS"),
            rm
        );
        print!(
            "{}: {}\n",
            cat_string("HTTP VERSION"),
            version
        );
        print!(
            "{}: {}\n",
            cat_string("REQUEST BODY"),
            body
        );
        print!(
            "{}: {}\n",
            cat_string("WAS TRUNCATED?:"),
            truncate
        );
        print!(
            "{}: {}\n",
            cat_string("SAFE MODE?"),
            safe_mode
        );
    }
    pub fn display_help() {
        let path = Path::new(&current_dir().expect("failed to get the current directory"))
            .join("help.txt");
        let contents = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => panic!("error when reading help file: {}", e),
        };
        print!("{}", contents);
    }
    pub fn display_final_message_success() {
        let msg = "YOUR REQUEST WAS SUCCESSFULLY MADE"
            .to_string()
            .green()
            .bold();
        print!(
            "{}: {}\n",
            cat_string("REQUEST RESULT"),
            msg
        );
    }
    pub fn display_final_message_failed(e: reqwest::Error) {
        let msg = format!("ERROR WHEN MAKING THE REQUEST: {}", e.to_string())
            .bold()
            .red();
        print!(
            "{}: {}\n",
            cat_string("REQUEST RESULT"),
            msg
        );
    }
}
