use crate::command::write_line;

use super::{CommandHandler, Context};

use async_trait::async_trait;

pub struct OptionHandler {}

#[async_trait]
impl CommandHandler for OptionHandler {
    fn name(&self) -> &'static str {
        "option"
    }

    async fn handle(&self, _: &Context, args: Vec<&str>) {
        // Sets the transport helper option <name> to <value>. 
        // Outputs a single line containing one of 
        // - ok (option successfully set)
        // - unsupported (option not recognized)
        // - or error <msg> (option <name> is supported but <value> is not valid for it).
        // Options should be set before other commands, 
        // and may influence the behavior of those commands.

        // Currently ok for every option.
        write_line("ok");
    }
}
