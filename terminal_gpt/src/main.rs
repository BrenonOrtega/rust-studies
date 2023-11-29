use std::error::Error;

use async_openai::{
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");

        let request: async_openai::types::CreateChatCompletionRequest = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
            .content(input)
            .build()?
            .into(),
        ]);
        
        println!("{}", serde_json::to_string(&request).unwrap());
        
        let response = client.chat().create(request).await?;
        
        println!("\nResponse:\n");
        for choice in response.choices {
            println!(
                "{}: Role: {}  Content: {:?}",
                choice.index, choice.message.role, choice.message.content
            );
        }
    }
        
    Ok(())
}