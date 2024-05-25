// request_scanner_submod encoders;
use log::error;
mod encoders;
pub mod decoders;

use crate::client::transport::ResponseIterator;
use crate::contracts::ContractDetails;
use crate::messages::{IncomingMessages, RequestMessage};
use crate::orders::TagValue;
use crate::{Client, Error};


pub type ScannerSubscriptionOptions = Vec<TagValue>;
pub type ScannerSubscriptionFilter = Vec<TagValue>;

pub struct ScannerSubscription {
    number_of_rows: i32,
    instrument: String,
    location_code: String,
    scan_code: String,
    above_price:  f64,
    below_price: f64,
    above_volume: i32,
    market_cap_above: f64,
    market_cap_below: f64,
    moody_rating_above: String,
    moody_rating_below: String,
    sp_rating_above: String,
    sp_rating_below: String,
    maturity_date_above: String,
    maturity_date_below: String,
    coupon_rate_above: f64,
    coupon_rate_below: f64,
    exclude_converiable: i32,
    average_options_volume_above: i32,
    scanner_settings_pairs: String,
    stock_type_filter: String
}

pub struct ScannerData {

    contract: ContractDetails,
    rank: i32,
    distance: String,
    benchmark: String,
    projection: String,
    legs: String,
}


pub(crate) struct ScannerSubscriptionIterator<'a> {
    client: &'a Client,
    request_id: i32,
    responses: ResponseIterator,
}

impl<'a> ScannerSubscriptionIterator<'a> {
    fn new(client: &'a Client, request_id: i32, responses: ResponseIterator) -> Self {

        Self { client, request_id, responses }
    }
}

impl<'a> Iterator for ScannerSubscriptionIterator<'a> {
    type Item = Vec<ScannerData>;

    fn next(&mut self) -> Option<Self::Item> {
            match self.responses.next() {
            Some(mut message) => match message.message_type() {

                IncomingMessages::ScannerData => {
                        let decoded = decoders::decode_scan_data_list(&mut message);
                        if let Ok(results) = decoded {
                            return Some(results);
                        }

                    error!("unexpected message: {:?}", decoded.err());
                    None
                        
                },
                    _ => {
                        error!("unexpected message {message:?}");
                        None
                    },
                    
                /* IncomingMessages::Error => Err(Error::Simple(message.peek_string(4))),
                _ => Err(Error::Simple(format!("unexpected message: {:?}", message.message_type()))), */
            },
                None => None,
            } 
    }
}

pub(crate) fn scanner_subscription<'a> (
    client: &'a Client,
    scanner_subscription: &ScannerSubscription,
        scanner_subscription_options: Option<&ScannerSubscriptionOptions>,
        scanner_subscription_filter: Option<&ScannerSubscriptionFilter>,
) -> Result<ScannerSubscriptionIterator<'a>, Error> {
    let filter = match scanner_subscription_filter { Some(filter) => filter.to_owned(), None => Vec::<TagValue>::default() };
    let options = match scanner_subscription_options { Some(options) => options.to_owned(), None => Vec::<TagValue>::default() };
    let request_id = client.next_request_id();
    let request = encoders::scanner_subscription(
        client.server_version,
        request_id, 
        scanner_subscription, 
        &filter,
        &options,
    )?;

    let responses= client.send_durable_request(request_id, request)?;

    Ok(ScannerSubscriptionIterator::new(client, request_id, responses))
    /*
    if let Some(mut message) = messages.next() {
        match message.message_type() {
            crate::messages::IncomingMessages::ScannerData => {
                todo!("decode scanner data");
            },
            IncomingMessages::Error => Err(Error::Simple(message.peek_string(4))),
            _ => Err(Error::Simple(format!("unexpected message: {:?}", message.message_type()))),
        }
    } else {
        Err(Error::Simple("did not receive subscription data response".into()))
    }
    */
}
