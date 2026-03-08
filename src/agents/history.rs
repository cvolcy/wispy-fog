use std::path::PathBuf;

use rig::message::Message;
use tokio::{fs::{File, OpenOptions}, io::{AsyncBufReadExt, AsyncWriteExt, BufReader}};

pub trait History {
    async fn add(&self, prompt: Message) -> anyhow::Result<String>;
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

#[cfg(test)]
mod tests {
    use super::{History, JSONLHistory};
    use rig::message::{AssistantContent, Message};
    use rig::OneOrMany;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_history_path() -> PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        path.push(format!("wispy_fog_history_test_{}.jsonl", nanos));
        path
    }

    fn assistant_message(text: &str) -> Message {
        Message::Assistant {
            id: None,
            content: OneOrMany::one(AssistantContent::text(text.to_string())),
        }
    }

    #[tokio::test]
    async fn jsonl_history_returns_latest_messages_in_order() -> anyhow::Result<()> {
        let path = temp_history_path();
        let history = JSONLHistory::new(&path);

        history.add(assistant_message("first")).await?;
        history.add(assistant_message("second")).await?;
        history.add(assistant_message("third")).await?;

        let messages = history.get(2).await?;
        let serialized: Vec<String> = messages
            .iter()
            .map(|message| serde_json::to_string(message).expect("serialize message"))
            .collect();

        let expected = vec![
            serde_json::to_string(&assistant_message("second"))
                .expect("serialize message"),
            serde_json::to_string(&assistant_message("third"))
                .expect("serialize message"),
        ];

        assert_eq!(serialized, expected);

        let _ = tokio::fs::remove_file(&path).await;
        Ok(())
    }
}