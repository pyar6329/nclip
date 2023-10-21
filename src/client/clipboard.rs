use super::*;

pub struct ClipboardClient;

#[derive(Serialize)]
struct GetRequestBody;

#[derive(Deserialize)]
struct GetResponseBody {
    status: u16,
    content: String,
}

#[derive(Serialize)]
struct PostRequestBody {
    content: String,
}

#[derive(Deserialize)]
struct PostResponseBody;

impl ClipboardClient {
    pub async fn get(client: &Client) -> Result<String, Error> {
        let request_body = GetRequestBody;
        let result: GetResponseBody = client
            .execute(HTTPMethod::GET, "/clipboards", &request_body)
            .await?;

        Ok(result.content)
    }

    pub async fn post(client: &Client, content: &str) -> Result<(), Error> {
        let request_body = PostRequestBody {
            content: content.to_string(),
        };
        let _: PostResponseBody = client
            .execute(HTTPMethod::POST, "/clipboards", &request_body)
            .await?;

        Ok(())
    }
}
