//
//  Copyright 2024 Ram Flux, LLC.
//

use fred::prelude::*;


pub async fn test_data(rd: &RedisPool) -> Result<(), RedisError> {
    println!("Test data");
    
    rd.set("mykey", "myvalue11", None, None, false).await?;


    let value: RedisValue = rd.get("mykey").await?;
    let value_str: String = value.convert()?;
    println!("Value: {}", value_str);

    Ok(())
}
