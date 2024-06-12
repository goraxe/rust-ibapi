// request_scanner_submod encoders;
use log::error;
pub mod decoders;
mod encoders;

use crate::client::transport::ResponseIterator;
use crate::contracts::ContractDetails;
use crate::messages::{IncomingMessages, RequestMessage};
use crate::orders::TagValue;
use crate::{Client, Error};

pub type ScannerSubscriptionOptions = Vec<TagValue>;
pub type ScannerSubscriptionFilter = Vec<TagValue>;

pub struct ScannerSubscription {
    pub number_of_rows: i32,
    pub instrument: String,
    pub location_code: String,
    pub scan_code: String,
    pub above_price: f64,
    pub below_price: f64,
    pub above_volume: i32,
    pub market_cap_above: f64,
    pub market_cap_below: f64,
    pub moody_rating_above: String,
    pub moody_rating_below: String,
    pub sp_rating_above: String,
    pub sp_rating_below: String,
    pub maturity_date_above: String,
    pub maturity_date_below: String,
    pub coupon_rate_above: f64,
    pub coupon_rate_below: f64,
    pub exclude_converiable: i32,
    pub average_options_volume_above: i32,
    pub scanner_settings_pairs: String,
    pub stock_type_filter: String,
}

impl Default for ScannerSubscription {
    fn default() -> Self {
        Self {
            number_of_rows: -1,
            instrument: "".to_string(),
            location_code: "".to_string(),
            scan_code: "".to_string(),
            above_price: f64::MAX,
            below_price: f64::MAX,
            above_volume: i32::MAX,
            market_cap_above: f64::MAX,
            market_cap_below: f64::MAX,
            moody_rating_above: "".to_string(),
            moody_rating_below: "".to_string(),
            sp_rating_above: "".to_string(),
            sp_rating_below: "".to_string(),
            maturity_date_above: "".to_string(),
            maturity_date_below: "".to_string(),
            coupon_rate_above: f64::MAX,
            coupon_rate_below: f64::MAX,
            exclude_converiable: i32::MAX,
            average_options_volume_above: i32::MAX,
            scanner_settings_pairs: "".to_string(),
            stock_type_filter: "".to_string(),
        }
    }
}

#[derive(Debug)]
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
        Self {
            client,
            request_id,
            responses,
        }
    }

    fn cancel(&mut self) -> Result<(), Error> {
        let message = encoders::cancel_scanner_subscription(self.request_id)?;
        self.client.send_message(message)
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
                }
                _ => {
                    error!("unexpected message {message:?}");
                    None
                }

                /*
                IncomingMessages::Error => Err(Error::Simple(message.peek_string(4))),
                _ => Err(Error::Simple(format!("unexpected message: {:?}", message.message_type()))),*/
            },
            None => None,
        }
    }
}

impl<'a> Drop for ScannerSubscriptionIterator<'a> {
    fn drop(&mut self) {
        self.cancel().unwrap();
    }
}

pub(crate) fn scanner_subscription<'a>(
    client: &'a Client,
    scanner_subscription: &ScannerSubscription,
    scanner_subscription_options: Option<&ScannerSubscriptionOptions>,
    scanner_subscription_filter: Option<&ScannerSubscriptionFilter>,
) -> Result<ScannerSubscriptionIterator<'a>, Error> {
    let filter = match scanner_subscription_filter {
        Some(filter) => filter.to_owned(),
        None => Vec::<TagValue>::default(),
    };
    let options = match scanner_subscription_options {
        Some(options) => options.to_owned(),
        None => Vec::<TagValue>::default(),
    };
    let request_id = client.next_request_id();
    let request = encoders::scanner_subscription(client.server_version, request_id, scanner_subscription, &filter, &options)?;

    let responses = client.send_durable_request(request_id, request)?;

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
