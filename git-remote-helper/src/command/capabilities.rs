use super::{CommandHandler, Context, write_line};

use async_trait::async_trait;

pub struct CapabilitiesHandler {
    pub capabilities: Vec<&'static str>,
}

impl CapabilitiesHandler {
    pub fn new(capabilities: Vec<&'static str>) -> Self {
        Self { capabilities }
    }
}

// Each capability may be preceded with *,
// which marks them mandatory for Git versions using the remote helper to understand.
#[async_trait]
impl CommandHandler for CapabilitiesHandler {
    fn name(&self) -> &'static str {
        "capabilities"
    }

    async fn handle(&self, _: &Context, _: Vec<&str>) {
        for capability in self.capabilities.iter() {
            write_line(capability);
        }

        write_line("");
    }
}
