use quiz_domain::services::quiz::QuizServiceInterface;

use crate::models::quiz::question::{ModelIDImpl, QuestionSetImpl};

pub struct QuizServiceImpl;

impl QuizServiceInterface<'_, ModelIDImpl, QuestionSetImpl> for QuizServiceImpl {}
