use crate::domain::services::quiz::QuizServiceInterface;
use crate::simple_domain_impl::models::quiz::question::{ModelIDImpl, QuestionSetImpl};

pub struct QuizServiceImpl;

impl QuizServiceInterface<'_, ModelIDImpl, QuestionSetImpl> for QuizServiceImpl {}
