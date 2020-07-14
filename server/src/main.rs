extern crate pretty_env_logger;

use quiz_domain::models::quiz::QuestionSetImpl;
use quiz_domain::services::quiz::QuizServiceImpl;
use server::App;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let (_app, future) = App::new::<QuestionSetImpl, QuizServiceImpl>();
    future.await;
}
