// API definition & playground:
//   http://localhost:3000 
// API endpoints
//   GET http://localhost:3000/api/health
//   GET http://localhost:3000/api/v1/hello
//   GET http://localhost:3000/api/v1/hello?name=world

use poem::{listener::TcpListener, Route};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};

// a container for our API
// (at some point in the future it will contain our internal state)
struct Api;

#[OpenApi]
impl Api {

    // every API should have a health check endpoint
    // (our API doesn't actually do anything yet and we will always return 'OK')
    //
    // curl http://localhost:3000/api/health
    #[oai(path = "/health", method = "get")]
    async fn health_status(&self) -> PlainText<String> {
        PlainText("OK".to_string())
    }

    // this is our first endpoint and most simplistic endpoint
    //
    // curl http://localhost:3000/api/v1/hello
    // curl http://localhost:3000/api/v1/hello?name=world
    #[oai(path = "/v1/hello", method = "get")]
    async fn say_hello(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }

}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_name = "A beginner's journey to Poem";
    let api_version = "1.0";
    let api = Api{};

    // black magic - ignore this section for now
    let api_service = OpenApiService::new(api, api_name, api_version).server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/", ui);
    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
