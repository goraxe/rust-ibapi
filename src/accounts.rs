use log::error;

use crate::client::transport::GlobalResponseIterator;
use crate::contracts::Contract;
use crate::messages::IncomingMessages;
use crate::{server_versions, Client, Error};

mod decoders;
mod encoders;

#[derive(Debug, Default)]
pub struct AccountSummary {
    /// Account ID
    pub account_id: String,
    /// Account summary tag
    pub tag: String,
    /// Account summary value
    pub value: String,
    /// Currency
    pub currency: String,
}

#[derive(Debug, Default)]
pub struct PnL {
    /// Daily PnL
    pub daily_pnl: f64,
    /// Unrealized PnL
    pub unrealized_pnl: f64,
    /// Realized PnL
    pub realized_pnl: f64,
}

#[derive(Debug, Default)]
pub struct Position {
    /// Account holding position
    pub account: String,
    /// Contract
    pub contract: Contract,
    /// Size of position
    pub position: f64,
    /// Average cost of position
    pub average_cost: f64,
}

#[derive(Debug, Default)]
pub struct FamilyCode {
    /// Account ID
    pub account_id: String,
    /// Family code
    pub family_code: String,
}

pub(crate) fn account_summary<'a>(client: &'a Client, group_name: &str, tags: &str) -> Result<impl Iterator<Item = AccountSummary> + 'a, Error> {
    client.check_server_version(server_versions::ACCOUNT_SUMMARY, "It does not support account summary requests.")?;

    let request_id = client.next_request_id();

    let message = encoders::request_account_summary(request_id, group_name, tags)?;

    let messages = client.request_account_summary(message)?;

    Ok(AccountSummaryIterator {
        client,
        messages,
        request_id,
    })
}

pub(crate) fn cancel_account_summary(client: &Client, request_id: i32) -> Result<(), Error> {
    client.check_server_version(server_versions::ACCOUNT_SUMMARY, "It does not support account summary cancellation.")?;

    let message = encoders::cancel_account_summary(request_id)?;

    client.request_account_summary(message)?;

    Ok(())
}

pub(crate) fn pnl(client: &Client, account_id: &str) -> Result<PnL, Error> {
    client.check_server_version(server_versions::ACCOUNT_SUMMARY, "It does not support single PNL requests.")?;

    let request_id = client.next_request_id();

    let message = encoders::request_pnl(request_id, account_id)?;

    let mut messages = client.request_pnl(message)?;

    if let Some(mut message) = messages.next() {
        cancel_pnl(client, request_id)?;
        decoders::decode_pnl(&mut message)
    } else {
        Ok(PnL::default())
    }
}

// Subscribes to position updates for all accessible accounts.
// All positions sent initially, and then only updates as positions change.
pub(crate) fn positions(client: &Client) -> Result<impl Iterator<Item = Position> + '_, Error> {
    client.check_server_version(server_versions::ACCOUNT_SUMMARY, "It does not support position requests.")?;

    let message = encoders::request_positions()?;

    let messages = client.request_positions(message)?;

    Ok(PositionIterator { client, messages })
}

pub(crate) fn cancel_pnl(client: &Client, request_id: i32) -> Result<(), Error> {
    client.check_server_version(server_versions::ACCOUNT_SUMMARY, "It does not support position cancellation.")?;

    let message = encoders::cancel_pnl(request_id)?;

    client.request_pnl(message)?;

    Ok(())
}

pub(crate) fn cancel_positions(client: &Client) -> Result<(), Error> {
    client.check_server_version(server_versions::ACCOUNT_SUMMARY, "It does not support position cancellation.")?;

    let message = encoders::cancel_positions()?;

    client.request_positions(message)?;

    Ok(())
}

// Determine whether an account exists under an account family and find the account family code.
pub(crate) fn family_codes(client: &Client) -> Result<Vec<FamilyCode>, Error> {
    client.check_server_version(server_versions::REQ_FAMILY_CODES, "It does not support family codes requests.")?;

    let message = encoders::request_family_codes()?;

    let mut messages = client.request_family_codes(message)?;

    if let Some(mut message) = messages.next() {
        decoders::decode_family_codes(&mut message)
    } else {
        Ok(Vec::default())
    }
}

// Supports iteration over [AccountSummary].
pub(crate) struct AccountSummaryIterator<'a> {
    client: &'a Client,
    request_id: i32,
    messages: GlobalResponseIterator,
}

impl<'a> Iterator for AccountSummaryIterator<'a> {
    type Item = AccountSummary;

    // Returns the next [AccountSummary]. Waits up to x seconds for next [AccountSummaryEnd].
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(mut message) = self.messages.next() {
                match message.message_type() {
                    IncomingMessages::AccountSummary => match decoders::decode_account_summary(&mut message) {
                        Ok(val) => return Some(val),
                        Err(err) => {
                            error!("error decoding account summary: {err}");
                        }
                    },
                    IncomingMessages::AccountSummaryEnd => {
                        if let Err(e) = cancel_account_summary(self.client, self.request_id) {
                            error!("error cancelling account summary: {e}")
                        }
                        return None;
                    }
                    message => {
                        error!("account summary iterator unexpected message: {message:?}");
                    }
                }
            } else {
                return None;
            }
        }
    }
}

// Supports iteration over [Position].
pub(crate) struct PositionIterator<'a> {
    client: &'a Client,
    messages: GlobalResponseIterator,
}

impl<'a> Iterator for PositionIterator<'a> {
    type Item = Position;

    // Returns the next [Position]. Waits up to x seconds for next [OrderDataResult].
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(mut message) = self.messages.next() {
                match message.message_type() {
                    IncomingMessages::Position => match decoders::decode_position(&mut message) {
                        Ok(val) => return Some(val),
                        Err(err) => {
                            error!("error decoding execution data: {err}");
                        }
                    },
                    IncomingMessages::PositionEnd => {
                        if let Err(e) = cancel_positions(self.client) {
                            error!("error cancelling positions: {e}")
                        }
                        return None;
                    }
                    message => {
                        error!("order data iterator unexpected message: {message:?}");
                    }
                }
            } else {
                return None;
            }
        }
    }
}
