use owo_colors::OwoColorize;
use xmas_trivia::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "http://localhost:8080";
    println!("Server running at {}", url.blue().underline());
    run()?.await
}
