use crate::{
    util::{
        directive::CloudPlatform,
        encryption::{generate_seeds, KEY_SIZE, NONCE_SIZE},
    },
    cloud_storage::drive,
};
use oauth2::{
    basic::BasicClient, 
    AccessToken, AuthUrl, ClientId, CsrfToken, 
    RedirectUrl, ResponseType, Scope
};
use serde::{Serialize,Deserialize};
use std::{
    env, 
    time::{SystemTime, UNIX_EPOCH},
    io::{BufRead, BufReader, Read, Write},
    net::TcpListener,
};

///Holds the user credentials for the session
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserToken {
    ///Platform the token is used for
    pub service: CloudPlatform,
    ///Key for en/decrypting the user token
    pub key_seed: [u8; KEY_SIZE],
    ///Nonce for en/decrypting the user token
    pub nonce_seed: [u8; NONCE_SIZE],
    ///Time stamp for the user token
    pub time_stamp: u64,
    ///Grants access to the user account
    pub access_token: String,
}

///Authenticate user with google and get access token for drive
pub fn google_access() -> anyhow::Result<UserToken> {
    // Set up the config for the Google OAuth2 process.
    let client = BasicClient::new(
        ClientId::new(drive::GOOGLE_CLIENT_ID.to_string()),
        None, //No secret for implicit flow
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .expect("Invalid authorization endpoint URL"),
        None,
    )
    .set_redirect_uri(
        //Use a local server to redirect
        RedirectUrl::new("http://localhost:3000".to_string()).expect("Invalid redirect URL"),
    );

    //Authorization URL to redirect the user
    let (authorize_url, _) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/drive".to_string(),
        ))
        .use_implicit_flow()
        .set_response_type(&ResponseType::new("token".to_string()))
        .url();

    println!(
        "Open this URL to authorize this application:\n{}\n",
        authorize_url
    );
    let mut token: Option<String> = None;

    //Redirect server that grabs the token
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            // Read the HTTP request
            let mut reader = BufReader::new(&stream);
            let mut request = String::new();
            reader.read_line(&mut request).unwrap();

            // Check for GET request and serve the HTML with JavaScript
            if request.starts_with("GET") {
                let html = r#"
                    <html>
                    <body>
                        <script>
                        window.onload = function() {
                            var hash = window.location.hash.substr(1);
                            var xhr = new XMLHttpRequest();
                            xhr.open("POST", "http://localhost:3000/token", true);
                            xhr.setRequestHeader("Content-Type", "application/x-www-form-urlencoded");
                            xhr.send(hash);
                        };
                        </script>
                        <p>You can now close this page and return to the applciation</p>
                    </body>
                    </html>
                "#;
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    html.len(),
                    html
                );
                stream.write_all(response.as_bytes()).unwrap();
            }
            // Check for POST request to /token
            else if request.starts_with("POST /token") {
                let mut content_length = 0;
                let mut headers = String::new();

                //read the line until breakpoint reached
                while reader.read_line(&mut headers).unwrap() > 0 {
                    //Get the length of the body
                    if headers.starts_with("Content-Length:") {
                        content_length = headers
                            .split_whitespace()
                            .nth(1)
                            .unwrap()
                            .parse::<usize>()
                            .unwrap();
                    }
                    //break out of the loop if end reached
                    if headers == "\r\n" {
                        break;
                    }
                    headers.clear();
                }
                //Read the body
                let mut body_buffer = vec![0_u8; content_length];
                reader.read_exact(&mut body_buffer).unwrap();
                let body = String::from_utf8(body_buffer).unwrap();

                //Extract the token
                token = body
                    .split('&')
                    .find(|param| param.starts_with("access_token"))
                    .and_then(|param| param.split('=').nth(1))
                    .map(str::to_string);

                //Respond to close connection
                let response = "HTTP/1.1 200 OK\r\n\r\n";
                stream.write_all(response.as_bytes()).unwrap();
                break; //shut down server
            }
        }
    }

    //Create the user_token
    let (key_seed, nonce_seed) = generate_seeds();
    let user_token = UserToken {
        service: CloudPlatform::Google,
        key_seed,
        nonce_seed,
        time_stamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Somehow, time has gone backwards")
            .as_secs(),
        access_token: token.unwrap().to_string(),
    };

    return Ok(user_token);
}

pub fn dropbox_access() {
    let client_id = "im68gew9aehy2pn".to_string();

    let client = BasicClient::new(
        ClientId::new(client_id),
        None,
        AuthUrl::new("https://www.dropbox.com/oauth2/authorize".to_string())
            .expect("Invalid authorization endpoint URL"),
        None,
    )
    .set_redirect_uri(RedirectUrl::new("http://localhost:3000".to_string()).unwrap());

    let (_authorize_url, _csrf_state) = client.authorize_url(CsrfToken::new_random).url();
}
