use scylla::transport::errors::QueryError;
use scylla::{Session, SessionBuilder};

pub struct CassandraRepository {
    session: Session,
}

impl CassandraRepository {
    pub async fn new(uri: &str) -> Self {
        let session = SessionBuilder::new()
            .known_node(uri)
            .build()
            .await
            .expect("Failed to connect to Cassandra");

        Self { session }
    }

    pub async fn insert_user(
        &self,
        user_id: &str,
        email: &str,
        created_at: i64,
    ) -> Result<(), QueryError> {
        self.session
            .query(
                "INSERT INTO users (user_id, email, created_at)
                 VALUES (?, ?, ?)",
                (user_id, email, created_at),
            )
            .await?;

        Ok(())
    }
}
