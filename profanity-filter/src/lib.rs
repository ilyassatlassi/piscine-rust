pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.contains("stupid")|| message.is_empty()  {
        return  Err("ERROR: illegal"); 
    }
    Ok(message)    
    // todo!()
}
