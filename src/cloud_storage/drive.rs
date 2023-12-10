use super::oauth::UserCredentials;
use reqwest;
use serde_json::{from_reader, Value};


pub async fn get_drive_info(credentials: UserCredentials) -> anyhow::Result<()> {
    //Token to query the drive
    let mut page_token: Option<String> = None;
    //Loop through each page
    loop {
        let url = match &page_token {
            Some(token) => {
                format!("https://www.googleapis.com/drive/v3/files?pageToken={}", token)
            }
            None => {
                "https://www.googleapis.com/drive/v3/files".to_string()
            }
        };
        
        let response = reqwest::Client::new()
            .get(url)
            .bearer_auth(&credentials.access_token)
            .send()
            .await?;

        if response.status().is_success() {
            let stuff = response.json::<Value>().await?;
            println!("{:#?}", &stuff);

            if let Some(next_token) = stuff["nextPageToken"].as_str() {
                page_token = Some(next_token.to_string());
            } else {
                break;
            }
        } else {
            println!("Error {:?}", response.text().await?);
            break;
        }
    }
    return Ok(());
}

async fn google_create_folder(credentials: UserCredentials) -> anyhow::Result<()> {


    return Ok(());
}
//Check if drive path exists
// - Name, MIME type, etc.

//Create folder if none
// - MIME type application/vnd.google-apps.folder
// - Create any subsequent folders

//Upload file