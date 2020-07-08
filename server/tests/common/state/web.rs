use anyhow::Result;
use http::Method;
use reqwest::header::HeaderMap;
use reqwest::ClientBuilder;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub(crate) struct RequestBuilder {
    method: Method,
    url: Option<String>,
    response: Option<Response>,
    headers: HeaderMap,
}

impl RequestBuilder {
    pub(crate) async fn send(&mut self) -> Result<()> {
        let url: String = self.url.clone().unwrap();
        let headers = self.headers.clone();

        let builder = match self.method {
            Method::GET => ClientBuilder::new().build()?.get(&url),
            _ => unimplemented!(),
        };
        let response = builder.headers(headers).send().await?;
        self.response = Some(Response::new(response).await);
        Ok(())
    }

    pub(crate) fn with_url(&mut self, url: String) -> &mut Self {
        self.url = Some(url);
        self
    }

    pub(crate) fn with_header(
        &mut self,
        key: reqwest::header::HeaderName,
        value: String,
    ) -> &mut Self {
        self.headers.append(key, value.parse().unwrap());
        self
    }

    pub(crate) fn without_header(&mut self, key: http::header::HeaderName) -> &mut Self {
        self.headers.remove(key);
        self
    }

    pub(crate) fn response(&mut self) -> &mut Option<Response> {
        &mut self.response
    }
}

impl Default for RequestBuilder {
    fn default() -> Self {
        RequestBuilder {
            method: Method::GET,
            url: None,
            response: None,
            headers: HeaderMap::default(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Response {
    status: StatusCode,
    headers: HeaderMap,
    body: String,
}

impl Response {
    async fn new(response: reqwest::Response) -> Response {
        Response {
            status: response.status(),
            headers: response.headers().clone(),
            body: response.text().await.unwrap(),
        }
    }

    pub(crate) async fn json<T: DeserializeOwned>(&self) -> Result<T> {
        let res = serde_json::from_str::<T>(self.body.as_ref())?;
        Ok(res)
    }

    pub(crate) fn status(&self) -> &StatusCode {
        &self.status
    }

    pub(crate) fn body(&self) -> &String {
        &self.body
    }
}
