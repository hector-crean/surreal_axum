extern crate serde;
use serde::{Deserialize, Serialize};

use crate::services::open_ai::{
    finish_reason::FinishReason, function::FunctionCall, role::OpenAiRole,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionResponseChoicesInner {
    /// The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence, `length` if the maximum number of tokens specified in the request was reached, `content_filter` if content was omitted due to a flag from our content filters, or `function_call` if the model called a function.
    #[serde(rename = "finish_reason")]
    pub finish_reason: FinishReason,
    /// The index of the choice in the list of choices.
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "message")]
    pub message: ChatCompletionMessage,
}

/// Represents a chat completion response returned by the model, based on the provided input.
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionResponse {
    /// A unique identifier for the chat completion.
    #[serde(rename = "id")]
    pub id: String,
    /// A list of chat completion choices. Can be more than one if `n` is greater than 1.
    #[serde(rename = "choices")]
    pub choices: Vec<ChatCompletionResponseChoicesInner>,
    /// The Unix timestamp (in seconds) of when the chat completion was created.
    #[serde(rename = "created")]
    pub created: i32,
    /// The model used for the chat completion.
    #[serde(rename = "model")]
    pub model: String,
    /// The object type, which is always `chat.completion`.
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

/// Represents a single choice among the chat completion choices.
#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    /// The reason the model stopped generating tokens.
    /// This will be `stop` if the model hit a natural stop point or a provided stop sequence,
    /// `length` if the maximum number of tokens specified in the request was reached,
    /// `content_filter` if content was omitted due to a flag from our content filters,
    /// or `function_call` if the model called a function.
    finish_reason: String,
    /// The index of the choice in the list of choices.
    index: i32,
    /// A chat completion message generated by the model.
    message: ChatCompletionMessage,
}

/// Represents a chat completion message generated by the model.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChatCompletionMessage {
    /// The contents of the message.
    #[serde(rename = "content", deserialize_with = "Option::deserialize")]
    pub content: Option<String>,
    #[serde(rename = "function_call", skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
    /// The role of the author of this message.
    #[serde(rename = "role")]
    pub role: OpenAiRole,
}

/// Represents usage statistics for the completion request.
#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    /// Number of tokens in the generated completion.
    completion_tokens: i32,
    /// Number of tokens in the prompt.
    prompt_tokens: i32,
    /// Total number of tokens used in the request (prompt + completion).
    total_tokens: i32,
}
