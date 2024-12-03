use redis::{Commands, RedisError};

pub fn function(key: &str, value: &str) -> Result<(), RedisError> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  con.set(key, value)?;

  Ok(())
}