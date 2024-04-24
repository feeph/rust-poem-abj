// API definition & playground:
//   http://localhost:3000 
// API endpoints
//   GET  http://localhost:3000/api/health
//   GET  http://localhost:3000/api/v1/hello
//   GET  http://localhost:3000/api/v1/hello?name=world
//   POST http://localhost:3000/api/v1/hello?phrase=Greetings

use poem::{listener::TcpListener, Route};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};
use std::sync::{atomic::AtomicU64, atomic::Ordering, Mutex};

struct Api {
    // numeric values can use an atomic data type
    count:  AtomicU64,
    // strings need to be wrapped in a Mutex
    phrase: Mutex<String>,
}

#[OpenApi]
impl Api {

    #[oai(path = "/health", method = "get")]
    async fn health_status(&self) -> PlainText<String> {
        PlainText("OK".to_string())
    }

    #[oai(path = "/v1/hello", method = "get")]
    async fn say_hello(&self, name: Query<Option<String>>) -> PlainText<String> {
        // increase the counter each time we greet someone
        // I have no idea what the ordering should be
        self.count.fetch_add(1, Ordering::SeqCst);
        let phrase = self.phrase.lock().unwrap();
        match name.0 {
            Some(name) => PlainText(format!("{}, {}!", phrase, name)),
            None => PlainText(format!("{}!", phrase)),
        }
    }

    #[oai(path = "/v1/count", method = "get")]
    async fn get_count(&self) -> PlainText<String> {
        // perform an atomic read and return the counter value
        // I have no idea what the ordering should be
        PlainText(self.count.load(Ordering::Relaxed).to_string())
    }

    #[oai(path = "/v1/phrase", method = "get")]
    async fn get_phrase(&self) -> PlainText<String> {
        PlainText(self.phrase.lock().unwrap().clone())
    }

    #[oai(path = "/v1/phrase", method = "post")]
    async fn set_phrase(&self, name: Query<Option<String>>) -> PlainText<String> {
        // TODO implement
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello".to_string()),
        }
    }

}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_name = "A beginner's journey to Poem";
    let api_version = "1.0";
    let api = Api{count: AtomicU64::new(0), phrase: Mutex::new(String::from("hello"))};

    // black magic - ignore this section for now
    let api_service = OpenApiService::new(api, api_name, api_version).server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/", ui);
    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
