use axum::{
    extract::State,
    response::{IntoResponse, Json, Response},
};
use open_ai_client::{
    self,
    chat::{
        request::{ChatCompletionMessage, ChatCompletionRequest},
        response::ChatCompletionResponse,
    },
    role::OpenAiRole,
};

use crate::{error, AppState};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateChat {
    content: String,
}

pub async fn create_chat(
    State(AppState { open_ai_client, .. }): State<AppState>,
    Json(CreateChat { content }): Json<CreateChat>,
) -> Result<Json<ChatCompletionResponse>, error::ApiError> {
    let msgs = vec![
        ChatCompletionMessage {
            role: OpenAiRole::System,
            content: Some(String::from(
                r#"
            You are a Socratic tutor. Use the following principles in responding to students:\n\n
             Ask thought-provoking, open-ended questions that challenge students' preconceptions and 
             encourage them to engage in deeper reflection and critical thinking.\n- Facilitate open 
             and respectful dialogue among students, creating an environment where diverse viewpoints 
             are valued and students feel comfortable sharing their ideas.\n- 
             Actively listen to students' responses, paying careful attention to their underlying thought 
             processes and making a genuine effort to understand their perspectives.\n- Guide students in 
             their exploration of topics by encouraging them to discover answers independently, rather than 
             providing direct answers, to enhance their reasoning and analytical skills.\n- Promote critical 
             thinking by encouraging students to question assumptions, evaluate evidence, and consider alternative 
             viewpoints in order to arrive at well-reasoned conclusions.\n- Demonstrate humility by acknowledging 
             your own limitations and uncertainties, modeling a growth mindset and exemplifying the value of 
             lifelong learning."#,
            )),
            function_call: None,
            name: None,
        },
        ChatCompletionMessage {
            role: OpenAiRole::User,
            content: Some(content),
            function_call: None,
            name: None,
        },
    ];

    let req = ChatCompletionRequest::new(String::from("gpt-3.5-turbo"), msgs)
        .temperature(0.8)
        .max_tokens(1024)
        .top_p(1.)
        .frequency_penalty(0.)
        .presence_penalty(0.);

    let resp = open_ai_client.chat(req).await?;

    println!("{:?}", &resp);

    Ok(Json(resp))
}
