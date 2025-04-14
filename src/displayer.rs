//this module will handle any kind of output to the terminal

//downloaded crates
use colored::*;
use std::{env::*, path::Path};

//local crates
use crate::types as interalTypes;

// WRAPPER FUNCTIONS FOR CONSISTENCY
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
//END



//this is the funciton to display the request data from the user in a readable way
pub fn display_request_data(data: &interalTypes::RequestData) {
    let welcome_message = title_string("\n\n----USER REQUEST----\n\n");
    let body = match data.body.is_empty(){
        true => not_filled_string("no body"),
        false =>filled_string(&data.body), 
    };
    let url = filled_string(&data.url);

    let xhead = if data.xheaders == "noheaders".to_string() {
        not_filled_string("no headers")
    } else if data.xheaders == "headers hidden".to_string() {
        warn_filled("headers hidden")
    } else {
        filled_string(&data.xheaders)
    };
    let paras = match data.params.is_empty(){
        true => not_filled_string("no params"),
        false =>filled_string(&data.params), 
    };
    let q = match data.querys.is_empty(){
        true => not_filled_string("no query params"),
        false =>filled_string(&data.querys), 
    };
    let t = filled_string(interalTypes::RequestType::request_to_string(&data.rtype).as_str());
    let safe_mode = warn_filled(data.safe_mode.to_string().to_uppercase().as_ref());

    print!("{}", welcome_message);
    print!("{}: {}\n", cat_string("REQUEST URL"), url);
    print!("{}: {}\n", cat_string("REQUEST BODY"), body);
    print!("{}: {}\n", cat_string("REQUEST HEADERS"), xhead);
    print!("{}: {}\n", cat_string("REQUEST PARAMS"), paras);
    print!("{}: {}\n", cat_string("REQUEST QUERY PARAMS"), q);
    print!("{}: {}\n", cat_string("REQUEST TYPE"), t);
    print!("{}: {}\n", cat_string("SAFE MODE?"), safe_mode);
}


pub fn display_response_data(data: &interalTypes::ReponseData) {
    let welcome_message = title_string("\n\n----RECIVED RESPONSE ----\n\n");
    let status = match data.status.as_ref(){
        "3" => warn_filled(&data.status),
        "4"|"5" => not_filled_string(&data.status),
        _ => filled_string(&data.status)
    };
    let clen =  if data.content_length == "0" {not_filled_string(&data.content_length)} else {filled_string(&data.content_length)};
     
    let headers = if !data.safe_view {filled_string(&data.headers)} else {warn_filled(&data.headers)};
    let rm = match data.remote_address.starts_with("failed"){
        true => not_filled_string(&data.remote_address),
        false => not_filled_string(&data.remote_address)
    };
    let version = match data.version.starts_with("failed"){
        true => not_filled_string(&data.version),
        false => not_filled_string(&data.version)
    };
    let body = if data.body.starts_with("no") {
        not_filled_string(&data.body)
    } else if data.body.starts_with("~!$"){
        warn_filled("body recived but longer than 500 words...")
    } else {
        filled_string(&data.body)
    };
    let truncate = warn_filled(data.truncate.to_string().to_uppercase().as_ref());
    let safe_mode = warn_filled(data.safe_view.to_string().to_uppercase().as_ref()); 
    print!("{}", welcome_message);
    print!("{}: {}\n", cat_string("REQUEST STATUS"), status);
    print!("{}: {}\n", cat_string("CONTENT_LENGTH"), clen);
    print!("{}: {}\n", cat_string("REQUEST HEADERS"), headers);
    print!("{}: {}\n", cat_string("REQUEST REMOTE ADDRESS"), rm);
    print!("{}: {}\n", cat_string("HTTP VERSION"), version);
    print!("{}: {}\n", cat_string("REQUEST BODY"), body);
    print!("{}: {}\n", cat_string("WAS TRUNCATED?:"), truncate);
    print!("{}: {}\n", cat_string("SAFE MODE?"), safe_mode);
}
pub fn display_help() {
    let path =
        Path::new(&current_dir().expect("failed to get the current directory")).join("help.txt");
    let contents = std::fs::read_to_string(path).expect("couldn't find the file with the help message?");
    print!("{}", contents);
}
pub fn display_final_message_success() {
    let msg = "YOUR REQUEST WAS SUCCESSFULLY MADE"
        .to_string()
        .green()
        .bold();
    print!("{}: {}\n", cat_string("REQUEST RESULT"), msg);
}
pub fn display_final_message_failed(e: crate::errors::ValidError) {
    let msg = format!(
        "ERROR WHEN MAKING THE REQUEST,  REASON: {:?} MESSAGE: {}",
        e.reason, e.message
    )
    .bold()
    .red();
    print!("{}: {}\n", cat_string("REQUEST RESULT"), msg);
}
