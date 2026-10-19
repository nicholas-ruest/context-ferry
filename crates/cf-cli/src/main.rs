use anyhow::{Context,Result};use cf_config::Config;
fn main()->Result<()>{let command=std::env::args().nth(1).unwrap_or_else(||"example".into());let config=Config::from_env().context("configuration")?;println!("{}",cf_api::run(&command,&config.memory_path).context("context ferry")?);Ok(())}
