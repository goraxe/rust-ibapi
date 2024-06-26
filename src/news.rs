use crate::{Client, Error};

#[derive(Clone, Debug)]
pub struct NewsProvider {
    pub code: String,
    pub name: String,
}

// https://interactivebrokers.github.io/tws-api/news.html

/// Historical News Headlines

/// Requesting News Articles

#[allow(dead_code)]
/// Requests news providers which the user has subscribed to.
pub fn news_providers(client: &Client) -> Result<Vec<NewsProvider>, Error> {
    // request = RequestNewsProvidersRequest::new()
    // packet = request.encode()
    // client.send_packet(packet)
    // packet = client.receive_packet(request_id)
    // ReceiveNewsProvidersResponse::decode(packet)
    print!("client: {client:?}");
    Err(Error::NotImplemented)
}

// :reqNewsArticle below.

// reqHistoricalNews

//reqNewsArticle s
