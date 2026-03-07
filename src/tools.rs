use std::sync::Arc;

pub struct ToolRegistry {
    tools: Vec<Arc<dyn Tool + Send + Sync>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    pub fn register_tool<T: Tool + Send + Sync + 'static>(&mut self, tool: T) {
        self.tools.push(Arc::new(tool));
    }

    /// Returns a slice of registered tools.
    pub fn tools(&self) -> &[Arc<dyn Tool + Send + Sync>] {
        &self.tools
    }
}

pub trait Tool {
    /// Human-readable name suitable for invocation.
    fn name(&self) -> &str;

    /// Brief description to show in help/registration logs.
    fn description(&self) -> &str;
}

pub struct EchoTool;

impl EchoTool {
    pub fn new() -> Self {
        EchoTool
    }
}

impl Tool for EchoTool {
    fn name(&self) -> &str {
        "echo"
    }

    fn description(&self) -> &str {
        "A simple tool that echoes back the input. Usage: echo <message>"
    }
}