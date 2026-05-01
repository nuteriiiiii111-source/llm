use rig::client::CompletionClient;
use rig::{completion::Prompt, providers::openai};
use rocket::http::Method;
use rocket::{get, routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[macro_use]
extern crate rocket;

#[get("/?<message>")]
async fn index(message: &str) -> String {
    dotenv::dotenv().ok();
    let kimi_api_key =
        std::env::var("KIMI_API_KEY").expect("KIMI_API_KEY environment variable must be set");

    let client = openai::CompletionsClient::builder()
        .api_key(&kimi_api_key)
        .base_url("https://api.moonshot.ai/v1")
        .build()
        .expect("Failed to create OpenAI client for Kimi");

    let agent = client
        .agent("kimi-k2.6")
        .preamble("You are a helpful assistant.")
        .build();

    let response = agent.prompt(message).await.unwrap();
    println!("Response send");

    response.to_string()
}

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:5173"]);
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        send_wildcard: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::build().attach(cors).mount("/", routes![index])
}
