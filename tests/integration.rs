use std::fs;
use wispy_fog::agents::history::History;

#[test]
fn registry_registers_tools_in_integration() {
    use wispy_fog::tools::{ToolRegistry, echo::EchoTool};

    let mut registry = ToolRegistry::new();
    assert_eq!(registry.len(), 0);

    registry.register_tool(EchoTool::new());

    assert_eq!(registry.len(), 1);
    assert_eq!(registry.tools().len(), 1);
}

#[test]
fn skillmd_parsing_integration() {
    use wispy_fog::tools::skillmd::SkillMD;
    use wispy_fog::config::Config;

    let tmp = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    let path = tmp.join(format!("wispy_fog_integration_skill_{}.md", nanos));

    let content = "---\nname: integration_skill\ndescription: Integration test\n---\nDo the thing\n";
    fs::write(&path, content).expect("write skillmd");

    let cfg = Config::default();
    let skill = SkillMD::from_file(&path, cfg).expect("parse skillmd");
    assert_eq!(skill.metadata.name, "integration_skill");

    let _ = fs::remove_file(&path);
}

// Additional tests moved from in-file unit tests to integration tests.
// These verify the same behavior but from the library's public surface.

#[tokio::test]
async fn echo_tool_returns_prefixed_message() {
    use wispy_fog::tools::echo::EchoTool;
    use wispy_fog::tools::echo::EchoArgs;
    use rig::tool::Tool;

    let tool = EchoTool::new();
    let result = tool
        .call(EchoArgs { message: "hello".to_string() })
        .await
        .expect("echo tool failed");

    assert_eq!(result, "Echo: hello");
}

#[tokio::test]
async fn read_file_rejects_parent_dir_and_definition() {
    use wispy_fog::tools::{read_file::ReadFileTool, read_file::ReadFileArgs};
    use rig::tool::Tool;

    let tool = ReadFileTool::new(std::path::Path::new("./"));
    let res = tool
        .call(ReadFileArgs { filename: ["..", "secret.txt"].join(std::path::MAIN_SEPARATOR.to_string().as_str()) })
        .await;
    assert!(res.is_err());

    let def = tool.definition(String::new()).await;
    assert_ne!(def.parameters, serde_json::Value::Null);
}

#[tokio::test]
async fn terminal_definition_non_null() {
    use wispy_fog::tools::terminal::TerminalTool;
    use rig::tool::Tool;

    let tool = TerminalTool::new(std::path::Path::new("./"));
    let def = tool.definition(String::new()).await;
    assert_ne!(def.parameters, serde_json::Value::Null);
}

#[tokio::test]
async fn write_file_behaviour_tests() {
    use wispy_fog::tools::{write_file::WriteFileTool, write_file::WriteFileArgs};
    use rig::tool::Tool;

    // write content
    let tool = WriteFileTool::new(std::path::Path::new("./"));
    let tmp = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    let path = tmp.join(format!("wispy_fog_write_file_test_{}.txt", nanos));
    let filename = path.to_string_lossy().to_string();

    let result = tool.call(WriteFileArgs { filename: filename.clone(), content: "hello world".to_string() }).await.expect("write file failed");
    assert!(result.contains(&filename));
    let content = std::fs::read_to_string(&path).expect("read file");
    assert_eq!(content, "hello world");
    let _ = std::fs::remove_file(&path);

    // bad extension
    let res = tool.call(WriteFileArgs { filename: ["..", "output", "tests", "output.bin"].join(std::path::MAIN_SEPARATOR.to_string().as_str()), content: "nope".to_string() }).await;
    assert!(res.is_err());

    // parent dir
    let res2 = tool.call(WriteFileArgs { filename: ["..", "output", "tests", "escape.txt"].join(std::path::MAIN_SEPARATOR.to_string().as_str()), content: "blocked".to_string() }).await;
    assert!(res2.is_err());
}

#[tokio::test]
async fn jsonl_history_integration() -> anyhow::Result<()> {
    use wispy_fog::agents::history::JSONLHistory;
    use rig::message::{AssistantContent, Message};
    use rig::OneOrMany;

    fn assistant_message(text: &str) -> Message {
        Message::Assistant { id: None, content: OneOrMany::one(AssistantContent::text(text.to_string())) }
    }

    let tmp = std::env::temp_dir();
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("time went backwards").as_nanos();
    let path = tmp.join(format!("wispy_fog_history_test_{}.jsonl", nanos));
    let history = JSONLHistory::new(&path);

    history.add(assistant_message("first")).await?;
    history.add(assistant_message("second")).await?;
    history.add(assistant_message("third")).await?;

    let messages = history.get(2).await?;
    let serialized: Vec<String> = messages.iter().map(|m| serde_json::to_string(m).expect("serialize message")).collect();
    let expected = vec![ serde_json::to_string(&assistant_message("second")).expect("serialize"), serde_json::to_string(&assistant_message("third")).expect("serialize") ];
    assert_eq!(serialized, expected);

    let _ = tokio::fs::remove_file(&path).await;
    Ok(())
}

#[test]
fn tool_registry_registers_tools() {
    use wispy_fog::tools::{ToolRegistry, write_file::WriteFileTool};

    let mut registry = ToolRegistry::new();
    assert_eq!(registry.len(), 0);
    registry.register_tool(wispy_fog::tools::echo::EchoTool::new());
    registry.register_tool(WriteFileTool::new(std::path::Path::new("./")));
    assert_eq!(registry.len(), 2);
}
