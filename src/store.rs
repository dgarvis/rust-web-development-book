use sqlx::postgres::{PgPoolOptions, PgPool, PgRow};
use sqlx::Row;
//use std::collections::HashMap;
//use std::sync::Arc; // automatic ref counter... allows for a pointer to be shared.
//use tokio::sync::RwLock; // version of RW lock that work in multi-thread env... unlike the std version
use handle_errors::Error;
use crate::types::{
    question::{Question, QuestionId, NewQuestion},
    answer::{Answer, NewAnswer, AnswerId},
};
use tracing::{Level};

#[derive(Debug, Clone)]
pub struct Store {
    /*pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,*/
    pub connection: PgPool, // not sure I like this as public :/
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        /*Store {
            questions: Arc::new(RwLock::new(Self::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
    }*/
	let db_pool = match PgPoolOptions::new()
	    .max_connections(5)
	    .connect(db_url).await {
		Ok(pool) => pool,
		Err(e) => panic!("Couldn't establish DB connection:{}", e),
	    };

	Store {
	    connection: db_pool,
	}
    }

    /*fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
}*/

    pub async fn get_questions(&self, limit: Option<i32>, offset: i32) -> Result<Vec<Question>, Error> {
	match sqlx::query("SELECT * from questions LIMIT $1 OFFSET $2")
	    .bind(limit)
	    .bind(offset)
	    .map(|row: PgRow| Question {
		id: QuestionId(row.get("id")),
		title: row.get("title"),
		content: row.get("content"),
		tags: row.get("tags"),
	    })
	    .fetch_all(&self.connection)
	    .await {
		Ok(questions) => Ok (questions),
		Err(e) => {
		    tracing::event!(Level::ERROR, "{:?}", e);
		    Err(Error::DatabaseQueryError)
		}
	    }
    }

    pub async fn add_question(
	&self,
	new_question: NewQuestion
    ) -> Result<Question, Error> {
	match sqlx::query(
	    "INSERT INTO questions (title, content, tags) VALUES ($1, $2, $3) RETURNING id, title, content, tags"
	)
	    .bind(new_question.title)
	    .bind(new_question.content)
	    .bind(new_question.tags)
	    .map(|row: PgRow| Question {
		id: QuestionId(row.get("id")),
		title: row.get("title"),
		content: row.get("content"),
		tags: row.get("tags"),
	    })
	    .fetch_one(&self.connection)
	    .await
	{
	    Ok(question) => Ok(question),
	    Err(e) => {
		tracing::event!(Level::ERROR, "{:?}", e);
		Err(Error::DatabaseQueryError)
	    },
	}
    }

    pub async fn update_question(
	&self,
	question: Question,
	question_id: i32
    ) -> Result<Question, Error> {

	match sqlx::query(
	    "UPDATE questions SET title = $1, content = $2, tags = $3 WHERE id = $4 RETURNING id, title, content, tags"
	)
	    .bind(question.title)
	    .bind(question.content)
	    .bind(question.tags)
	    .bind(question_id)
	    .map(|row: PgRow| Question {
		id: QuestionId(row.get("id")),
		title: row.get("title"),
		content: row.get("content"),
		tags: row.get("tags"),
	    })
	    .fetch_one(&self.connection)
	    .await
	{
	    Ok(question) => Ok(question),
	    Err(e) => {
		tracing::event!(Level::ERROR, "{:?}", e);
		Err(Error::DatabaseQueryError)
	    },
	}
    }

    pub async fn delete_question(
	&self,
	question_id: i32
    ) -> Result<bool, Error> {

	match sqlx::query(
	    "DELETE FROM questions WHERE id = $1"
	)
	    .bind(question_id)
	    .execute(&self.connection)
	    .await
	{
	    Ok(_) => Ok(true),
	    Err(e) => {
		tracing::event!(Level::ERROR, "{:?}", e);
		Err(Error::DatabaseQueryError)
	    },
	}
    }

    pub async fn add_answer(
	&self,
	new_answer: NewAnswer
    ) -> Result<Answer, Error> {
	match sqlx::query("INSERT INTO answers (content, question_id) VALUES ($1, $2)")
	    .bind(new_answer.content)
	    .bind(new_answer.question_id.0)
	    .map(|row: PgRow| Answer {
		id: AnswerId(row.get("id")),
		content: row.get("content"),
		question_id: QuestionId(row.get("question_id")),
	    })
	    .fetch_one(&self.connection)
	    .await
	{
	    Ok(answer) => Ok(answer),
	    Err(e) => {
		tracing::event!(Level::ERROR, "{:?}", e);
		Err(Error::DatabaseQueryError)
	    },
	}
    }
    
    
}
