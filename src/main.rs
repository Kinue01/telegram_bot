use std::io::Error;
use std::sync::Arc;
use dotenvy::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use teloxide::prelude::*;
use teloxide::types::{InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub model_uri: String,
    pub completion_options: CompletionOptions,
    pub messages: Vec<YaMessage>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompletionOptions {
    pub stream: bool,
    pub temperature: String,
    pub max_tokens: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YaMessage {
    pub role: String,
    pub text: String,
}

async fn yandex_answer(msg: String) -> Result<String, Error> {
    let client = Client::new();

    let body = Root {
        model_uri: "gpt://b1gqker2ujkgd43bqipg/yandexgpt/latest".parse().unwrap(),
        completion_options: CompletionOptions {
            stream: true,
            temperature: "1.5".to_string(),
            max_tokens: "2000".to_string(),
        },
        messages: vec![YaMessage {
            role: "assistant".to_string(),
            text: msg,
        }],
    };

    log::log!(log::Level::Info, "Send to yandex");
    
    let res = client.post("https://llm.api.cloud.yandex.net/foundationModels/v1/completionAsync")
        .body(reqwest::Body::from(serde_json::to_string(&body).unwrap())).send().await.unwrap();

    log::log!(log::Level::Info, "Yandex answered");

    Ok(res.text().await.unwrap_or_default())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let bot = Bot::from_env();
    
    let handler = Update::filter_inline_query().branch(dptree::endpoint(
        |bot: Bot, q: InlineQuery| async move {
            let ya_search = InlineQueryResultArticle::new(
                "01".to_string(),
                "YandexGPT",
                InputMessageContent::Text(InputMessageContentText::new(
                    yandex_answer(q.query).await.unwrap()
                ))
            );

            let results = vec![
                InlineQueryResult::Article(ya_search),
            ];

            let r = bot.answer_inline_query(&q.id, results).send().await;

            log::log!(log::Level::Info, "Bot answered");

            match r {
                Ok(res) => respond(()),
                Err(err) => Ok(log::log!(log::Level::Error, "err"))
            }
        },
    ));

    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;
}
