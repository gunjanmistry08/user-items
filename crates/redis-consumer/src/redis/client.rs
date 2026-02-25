use redis::AsyncCommands;

pub struct RedisClient {
    client: redis::Client,
}

impl RedisClient {
    pub fn new(url: &str) -> Self {
        let client = redis::Client::open(url).expect("Invalid Redis URL");
        Self { client }
    }

    pub async fn set_user(&self, user_id: &str, payload: &str) -> redis::RedisResult<()> {
        let mut con = self.client.get_async_connection().await?;
        con.set(user_id, payload).await
    }
}
