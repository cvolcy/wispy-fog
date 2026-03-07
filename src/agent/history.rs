use std::path::PathBuf;

use rig::message::Message;
use tokio::{fs::{File, OpenOptions}, io::{AsyncBufReadExt, AsyncWriteExt, BufReader}};

pub trait History {
    async fn add(&self, prompt: Message) -> anyhow::Result<String>;
    async fn clear(&self) -> anyhow::Result<()>;
    async fn get(&self, count: usize) -> anyhow::Result<Vec<Message>>;
}

pub struct JSONLHistory {
    path: PathBuf,
}

impl JSONLHistory {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        JSONLHistory { path: path.into() }
    }
}

impl History for JSONLHistory {
    async fn add(&self, prompt: Message) -> anyhow::Result<String> {
        let mut file= OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .await?;

        let json = serde_json::to_string(&prompt)?;
        file.write_all(json.as_bytes()).await?;
        file.write_all(b"\n").await?;

        Ok(json)
    }

    async fn clear(&self) -> anyhow::Result<()> {
        tokio::fs::write(&self.path, b"").await?;
        Ok(())
    }

    async fn get(&self, count: usize) -> anyhow::Result<Vec<Message>> {
        let file = File::open(&self.path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let mut messages = Vec::new();

        while let Some(line) = lines.next_line().await? {
            if let Ok(msg) = serde_json::from_str::<Message>(&line) {
                messages.push(msg);
            }
        }

        Ok(messages.into_iter().rev().take(count).rev().collect())
    }
}