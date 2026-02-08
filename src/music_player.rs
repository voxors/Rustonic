use bytes::Bytes;
use futures_util::StreamExt;
use rodio::OutputStream;
use std::{error::Error, io::Cursor};

pub struct MusicPlayer {
    stream: Option<OutputStream>,
}

impl MusicPlayer {
    pub fn new() -> Self {
        MusicPlayer { stream: None }
    }

    pub async fn play(
        &mut self,
        mut stream: impl futures_core::Stream<Item = Result<Bytes, reqwest::Error>> + std::marker::Unpin,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut data: Vec<u8> = Vec::new();
        while let Some(chunk) = stream.next().await {
            if let Ok(chunk) = chunk {
                data.extend_from_slice(&chunk);
            }
        }
        let reader = Cursor::new(data);

        if self.stream.is_none() {
            self.stream = Some(rodio::OutputStreamBuilder::open_default_stream()?);
        }

        let mixer = self.stream.as_ref().unwrap().mixer();
        let sink = rodio::play(mixer, reader)?;
        sink.detach();

        Ok(())
    }
}
