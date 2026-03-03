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
        name: &str,
    ) -> Result<(), QueryError> {
        self.session
            .query(
                "INSERT INTO users.users (id, email, name) VALUES (?, ?, ?)",
                (user_id, email, name),
            )
            .await?;
        Ok(())
    }
}
