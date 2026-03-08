use rig::tool::Tool;

pub mod echo;
pub mod write_file;

trait AnyTool: Send + Sync {
    fn to_dyn(&self) -> Box<dyn rig::tool::ToolDyn>;
}

impl<T: Tool + Clone + Send + Sync + 'static> AnyTool for T {
    fn to_dyn(&self) -> Box<dyn rig::tool::ToolDyn> {
        Box::new(self.clone()) as Box<dyn rig::tool::ToolDyn>
    }
}

pub struct ToolRegistry {
    tools: Vec<Box<dyn AnyTool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    pub fn register_tool<T: Tool + Clone + Send + Sync + 'static>(&mut self, tool: T) {
        self.tools.push(Box::new(tool));
    }

    pub fn tools(&self) -> Vec<Box<dyn rig::tool::ToolDyn>> {
        self.tools.iter().map(|t| t.to_dyn()).collect()
    }
}