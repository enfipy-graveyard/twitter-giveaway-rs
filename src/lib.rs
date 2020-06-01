pub fn get_retweets() {
    let api_key = std::env::var("API_KEY").expect("API_KEY should be set");
    let api_secret_key = std::env::var("API_SECRET_KEY").expect("API_SECRET_KEY should be set");
    println!(
        "API key: {:?}, API secret key: {:?}",
        api_key, api_secret_key,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        get_retweets();
    }
}
