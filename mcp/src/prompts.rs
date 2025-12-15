use rust_mcp_schema::{ContentBlock, GetPromptResult, Prompt, PromptMessage, Role, TextContent};

macro_rules! define_prompts {
    (
        $(
            $fn_name:ident => {
                description: $description:literal,
                title: $title:literal,
                content: $content:expr,
            }
        ),+ $(,)?
    ) => {
        pub fn get_all() -> Vec<Prompt> {
            vec![
                $(
                    Prompt {
                        arguments: vec![],
                        description: Some($description.to_owned()),
                        meta: None,
                        name: stringify!($fn_name).to_owned(),
                        title: Some($title.to_owned()),
                    },
                )+
            ]
        }

        pub fn call(name: &str) -> Option<GetPromptResult> {
            match name {
                $(
                    stringify!($fn_name) => Some(GetPromptResult {
                        description: None,
                        messages: vec![PromptMessage {
                            content: ContentBlock::TextContent(TextContent::new(
                                $content.to_owned(),
                                None,
                                None,
                            )),
                            role: Role::User,
                        }],
                        meta: None,
                    }),
                )+
                _ => None,
            }
        }
    };
}

define_prompts! {
    plan_solana_program_stylus_migration => {
        description: "Prompts an LLM agent to plan for Solana program migration with the aid of the StylusPort::Solana handbook and MPC server",
        title: "Plan",
        content: include_str!("prompts/plan_solana_program_stylus_migration.md"),
    },
    execute_solana_program_stylus_migration => {
        description: "Prompts an LLM agent to execute a migration plan previously created by the planning prompt",
        title: "ExecutePlan",
        content: include_str!("prompts/execute_solana_program_stylus_migration.md"),
    },
}
