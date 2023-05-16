//this will be the package that will change the format of user data
//to be in a usable form for me

//NOTE might replace the dummy values with option returns then check for none
//when the fucnions are called

//split up the files into smaller files so it is more readable
//maybe like a display file to disply text and a types folder to hold the custuom types

pub mod format {
    //finish the main file to make the and display the request then recive the
    //response then display it,
    //will need to fix the "make_request" funtion to display the response instead of
    //just returning the unreadable string

    //create a type for the response type so it can be displayed
    // in a colorful and readable fashion
    use clap::Parser;

    #[derive(Debug)]
    pub struct ReponseData {
        pub status: String,
        pub content_length: String,
        pub headers: String,
        pub remote_address: String,
        pub version: String,
        pub body: String,
        pub truncate: bool,
    }
    fn version_to_string(v: reqwest::Version) -> String {
        match v {
            reqwest::Version::HTTP_09 => "HTTP/0.9".to_string(),
            reqwest::Version::HTTP_10 => "HTTP/1.0".to_string(),
            reqwest::Version::HTTP_11 => "HTTP/1.1".to_string(),
            reqwest::Version::HTTP_2 => "HTTP/2.0".to_string(),
            reqwest::Version::HTTP_3 => "HTTP/3.0".to_string(),
            _ => "failed when getting version".to_string(),
        }
    }
    //this will be the funcion to take in response and build it to
    // the custom type so it can be displayed in a readable fashion
    pub fn build_response_data(data: reqwest::blocking::Response, truncate: bool) -> ReponseData {
        //match statements are kinda awsesome
        ReponseData {
            status: data.status().to_string(),
            content_length: match data.content_length() {
                Some(c) => c.to_string(),
                //if error then the length of the content must be 0
                None => "0".to_string(),
            },
            headers: crate::formatters::format::headermap_to_string(data.headers()),
            remote_address: match data.remote_addr() {
                Some(a) => a.to_string(),
                None => "failed to find the servers address".to_string(),
            },
            version: crate::formatters::format::version_to_string(data.version()),
            body: match data.text() {
                Ok(b) => {
                    if truncate && b.len() >= 500 {
                        "body recived but longer than 500 words...".to_string()
                    } else {
                        b
                    }
                }
                Err(_) => "no body in response".to_string(),
            },
            truncate,
        }
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

    #[derive(Debug)]
    pub struct JsonTypes {
        pub key: String,
        pub value: String,
    }
    //make a funtion to add the regular params to the url
    //make a funtion to add the query params to the url, also need format
    //the qeuery params to the proper format
    #[derive(Debug, Clone)]
    pub enum RequestType {
        GET,
        DELETE,
        POST,
        PUT,
    }
    impl RequestType {
        //will convert the users input to the required value
        pub fn string_to_val(input: &str) -> RequestType {
            match input {
                "get" => RequestType::GET,
                "delete" => RequestType::DELETE,
                "post" => RequestType::POST,
                "put" => RequestType::PUT,
                _ => panic!("invalide request type given"),
            }
        }
        //will convert the enum to a string
        //dont have to give a defalut pattern becuse the enum wont allow any non
        //specified value to be passed to the funtion(gotta love static type checking)
        pub fn request_to_string(input: RequestType) -> String {
            match input {
                RequestType::GET => "GET".to_string(),
                RequestType::DELETE => "DELETE".to_string(),
                RequestType::POST => "POST".to_string(),
                RequestType::PUT => "PUT".to_string(),
            }
        }
    }

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
    }

    pub fn parse_json_types(head: String) -> Vec<JsonTypes> {
        // let mut map = HeaderMap::new();
        let mut harray: Vec<JsonTypes> = Vec::new();
        if head == "noheaders".to_string() || head == "".to_string() {
            return harray;
        }
        let head = head.replace("{", "");
        let head = head.replace("}", "");
        let h: Vec<String> = head.split(",").into_iter().map(|c| c.into()).collect();
        // let _ = head
        //     .split(",")
        //     .into_iter()
        //     .map(|val| val.split_once(":").unwrap())
        //     .into_iter()
        //     .clone()
        //     .map(|curr| map.insert(curr.0, curr.1.parse().unwrap()));
        // h.into_iter().map(|x| print!("\n{:?}\n", h[1].0));
        let xx: Vec<_> = h
            .iter()
            .map(move |val| {
                val.split_once(":")
                    .expect("invalid json type given to be parsed")
            })
            // .map(move |val| val.split_once(":").unwrap())
            .collect();
        for val in xx.into_iter() {
            let w = JsonTypes {
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
    fn format_query_params(query_params: Vec<JsonTypes>) -> String {
        //NOTE could build a vec of the string vesions of the params then
        //iterate throught that vec and create the final url if this doesnt work
        //this is how a query param should start
        let mut final_url = "/?".to_string();
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
    pub fn to_header_map(input: Vec<JsonTypes>) -> reqwest::header::HeaderMap {
        let mut map = reqwest::header::HeaderMap::new();
        //why cant i make this a one liner!!!
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
    #[derive(Debug, Clone)]
    pub struct RequestData {
        pub body: String,
        pub url: String,
        pub xheaders: String,
        pub params: String,
        pub querys: String,
        pub rtype: RequestType,
        pub truncate: bool,
    }
    //this will be a function that will add the query params and regular
    //params to the url before the request is made
    pub fn build_url(base_url: String, query_params: Vec<JsonTypes>, params: String) -> String {
        let query_params = crate::formatters::format::format_query_params(query_params);
        let params = crate::formatters::format::format_regular_parmas(params);
        format!("{}{}{}", base_url, params, query_params)
    }
    //if the full url is present just make the request without bothering with the rest
    pub fn build_request_data(args: ClientArgs) -> RequestData {
        //wtf this syntax is great
        RequestData {
            body: match args.body {
                Some(b) => b,
                None => "".to_string(),
            },
            url: match args.url {
                //only add the https to the front of domains not full urls
                Some(u) => u,
                None => format!(
                    "{}{}",
                    "http://",
                    args.domain
                        .expect("neither full url or domain name was passed")
                ),
            },
            xheaders: match args.xheaders {
                Some(x) => x,
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
                Some(t) => crate::formatters::format::RequestType::string_to_val(t.as_str()),
                None => crate::formatters::format::RequestType::GET,
            },
            truncate: args.truncate,
        }
    }
}