use redis::{Commands, RedisError};

pub fn function(key: &str) -> Result<String, RedisError> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  match con.get(key)? {
      Some(value) => Ok(value),
      None => Ok(String::from("Key not found"))
  }
}