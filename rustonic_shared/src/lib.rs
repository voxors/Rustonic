use std::error::Error;

use reqwest::{Client, Url};

const VERSION: &str = "1.16.1";
const BASE_PATH: &str = "rest";

#[derive(Debug, Clone)]
pub struct SubsonicApi {
    username: String,
    password: String,
    url: Url,
    client: Client,
    app_name: String,
}

impl SubsonicApi {
    pub fn new(
        username: String,
        password: String,
        url: String,
        app_name: String,
    ) -> Result<SubsonicApi, Box<dyn Error>> {
        Ok(SubsonicApi {
            username,
            password,
            url: Url::parse(url.as_str())?,
            client: Client::new(),
            app_name,
        })
    }

    pub async fn ping(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut ping_url = self.url.clone();

        let salt = "abcdefghij";
        let compute_string = &mut self.password.clone();
        compute_string.push_str(salt);
        let token = md5::compute(compute_string);

        ping_url
            .path_segments_mut()
            .map(|mut segs| {
                segs.pop_if_empty().push(BASE_PATH).push("ping");
            })
            .map_err(|_| "failed to set path segments")?;

        ping_url
            .query_pairs_mut()
            .append_pair("u", &self.username)
            .append_pair("t", format!("{:x}", token).as_str())
            .append_pair("s", salt)
            .append_pair("v", VERSION)
            .append_pair("c", &self.app_name);

        let response = self.client.get(ping_url).send().await?;
        println!("{} {:?}", response.status(), response.bytes().await?);

        Ok(())
    }
}
