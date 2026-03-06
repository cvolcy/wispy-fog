pub struct ToolRegistry {
    tools: Vec<Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    pub fn register_tool(&mut self, tool: Box<dyn Tool>) {
        self.tools.push(tool);
    }

    pub fn get_tools(&self) -> &Vec<Box<dyn Tool>> {
        &self.tools
    }
}

pub trait Tool {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

pub struct EchoTool;

impl Tool for EchoTool {
    fn name(&self) -> &str {
        "echo"
    }

    fn description(&self) -> &str {
        "A simple tool that echoes back the input. Usage: echo <message>"
    }
}