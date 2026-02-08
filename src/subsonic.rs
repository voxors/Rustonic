use bytes::Bytes;
use log::debug;
use reqwest::{Client, Url};
use std::error::Error;

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
    ) -> Result<SubsonicApi, Box<dyn Error + Send + Sync>> {
        Ok(SubsonicApi {
            username,
            password,
            url: Url::parse(url.as_str())?,
            client: Client::new(),
            app_name,
        })
    }

    fn get_base_url(&self) -> Url {
        let mut url = self.url.clone();
        let salt = "abcdefghij";
        let compute_string = &mut self.password.clone();
        compute_string.push_str(salt);
        let token = md5::compute(compute_string);
        url.query_pairs_mut()
            .append_pair("u", &self.username)
            .append_pair("t", format!("{:x}", token).as_str())
            .append_pair("s", salt)
            .append_pair("v", VERSION)
            .append_pair("c", &self.app_name);
        url
    }

    pub async fn ping(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut ping_url = self.get_base_url();
        ping_url
            .path_segments_mut()
            .map(|mut segs| {
                segs.pop_if_empty().push(BASE_PATH).push("ping");
            })
            .map_err(|_| "failed to set path segments")?;

        let response = self.client.get(ping_url).send().await?;
        debug!("{} {:?}", response.status(), response.bytes().await?);

        Ok(())
    }

    pub async fn stream(
        &self,
    ) -> Result<
        impl futures_core::Stream<Item = Result<Bytes, reqwest::Error>>,
        Box<dyn Error + Send + Sync>,
    > {
        let mut stream_url = self.get_base_url();
        stream_url
            .path_segments_mut()
            .map(|mut segs| {
                segs.pop_if_empty().push(BASE_PATH).push("stream");
            })
            .map_err(|_| "failed to set path segments")?;
        stream_url
            .query_pairs_mut()
            .append_pair("id", "LR1DVfn4696job8ZqTjWJU");

        let response = self.client.get(stream_url).send().await?;
        debug!("{}", response.status());

        Ok(response.bytes_stream())
    }
}
