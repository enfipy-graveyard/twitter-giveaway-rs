use egg_mode::{error::Result, user::TwitterUser, user::UserID, KeyPair, Response, Token};
use futures::TryStreamExt;

pub fn get_token(
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_secret: String,
) -> Token {
    Token::Access {
        consumer: KeyPair::new(consumer_key, consumer_secret),
        access: KeyPair::new(access_token, access_secret),
    }
}

pub async fn get_token_user(token: &Token) -> Result<TwitterUser> {
    let res: Response<TwitterUser> = egg_mode::verify_tokens(token).await?;
    println!("### Rate limit: {:?}", res.rate_limit_status);
    Ok(res.response)
}

pub async fn get_user(id: u64, token: &Token) -> Result<TwitterUser> {
    let res: Response<TwitterUser> = egg_mode::user::show(UserID::ID(id), token).await?;
    println!("### Rate limit: {:?}", res.rate_limit_status);
    Ok(res.response)
}

pub async fn get_retweeters(id: u64, token: &Token) -> Result<()> {
    let mut ids = vec![];
    let mut cursor = egg_mode::tweet::retweeters_of(id, &token);

    cursor.next_cursor = 1936684908;
    // cursor.page_size = Some(5);
    let mut val = cursor.call().await?.response;
    println!(
        "{:?}, {:?}, {:?}",
        val.ids, val.previous_cursor, val.next_cursor
    );
    ids.append(&mut val.ids);

    cursor.next_cursor = -1;
    // cursor.page_size = Some(5);
    let mut val = cursor.call().await?.response;
    println!(
        "{:?}, {:?}, {:?}",
        val.ids, val.previous_cursor, val.next_cursor
    );
    ids.append(&mut val.ids);

    // println!("{:?}", ids);
    Ok(())
}

pub async fn get_retweeters_map(id: u64, token: &Token) -> Result<()> {
    let mut req = egg_mode::tweet::retweeters_of(id, &token);
    // req.next_cursor = 1267562076920254465;
    req.page_size = Some(2);
    let retweeters: Vec<u64> = req.map_ok(|r| r.response).try_collect().await?;
    println!("{:?}", retweeters);

    let mut req = egg_mode::tweet::retweeters_of(id, &token);
    // req.next_cursor = 1187723232784326656;
    req.page_size = Some(1);
    let retweeters: Vec<u64> = req.map_ok(|r| r.response).try_collect().await?;
    println!("{:?}", retweeters);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::LevelFilter;

    #[tokio::test]
    async fn test_with_env() {
        pretty_env_logger::formatted_builder()
            .filter_level(LevelFilter::Info)
            .init();

        let consumer_key = std::env::var("CONSUMER_KEY").expect("CONSUMER_KEY should be set");
        let consumer_secret =
            std::env::var("CONSUMER_SECRET").expect("CONSUMER_SECRET should be set");
        let access_token = std::env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN should be set");
        let access_secret = std::env::var("ACCESS_SECRET").expect("ACCESS_SECRET should be set");

        let con_token = KeyPair::new(consumer_key, consumer_secret);
        // let con_token = KeyPair::new(access_token, access_secret);
        let request_token = egg_mode::request_token(&con_token, "oob").await.unwrap();
        // let url = egg_mode::authenticate_url(&request_token);
        let url = egg_mode::authorize_url(&request_token);
        println!("{}", url);
        // let pass = "bla-bla";
        // let (token, user_id, screen_name) =
        //     egg_mode::access_token(con_token, &request_token, &pass)
        //         .await
        //         .unwrap();

        // let token = get_token(consumer_key, consumer_secret, access_token, access_secret);
        // println!("Token: {:?}", token);
        // let user = get_token_user(&token).await.unwrap();
        // println!("User: {:?}", user.screen_name);

        // get_retweeters(1263974008900382721, &token).await.unwrap();
        // get_retweeters_map(1263974008900382721, &token)
        //     .await
        //     .unwrap();
    }
}
