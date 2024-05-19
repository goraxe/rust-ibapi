use crate::messages::OutgoingMessages;
use crate::messages::RequestMessage;
use crate::Error;

pub(crate) fn request_account_summary(request_id: i32, group_name: &str, tags: &str) -> Result<RequestMessage, Error> {
    let mut message = RequestMessage::new();
    let version = 1;

    message.push_field(&OutgoingMessages::RequestAccountSummary);
    message.push_field(&version);
    message.push_field(&request_id);
    message.push_field(&group_name);
    message.push_field(&tags);

    Ok(message)
}

pub(crate) fn cancel_account_summary(request_id: i32) -> Result<RequestMessage, Error> {
    let mut message = RequestMessage::new();

    message.push_field(&OutgoingMessages::CancelAccountSummary);
    message.push_field(&1);
    message.push_field(&request_id);

    Ok(message)
}

pub(crate) fn request_pnl(request_id: i32, account_id: &str) -> Result<RequestMessage, Error> {
    let mut message = RequestMessage::new();

    message.push_field(&OutgoingMessages::RequestPnL);
    message.push_field(&request_id);
    message.push_field(&account_id);
    message.push_field(&"");

    Ok(message)
}

pub(crate) fn cancel_pnl(request_id: i32) -> Result<RequestMessage, Error> {
    let mut message = RequestMessage::new();

    message.push_field(&OutgoingMessages::CancelPnL);
    message.push_field(&request_id);

    Ok(message)
}

pub(crate) fn request_positions() -> Result<RequestMessage, Error> {
    encode_simple(OutgoingMessages::RequestPositions, 1)
}

fn encode_simple(message_type: OutgoingMessages, version: i32) -> Result<RequestMessage, Error> {
    let mut message = RequestMessage::new();

    message.push_field(&message_type);
    message.push_field(&version);

    Ok(message)
}

pub(crate) fn cancel_positions() -> Result<RequestMessage, Error> {
    encode_simple(OutgoingMessages::CancelPositions, 1)
}

pub(crate) fn request_family_codes() -> Result<RequestMessage, Error> {
    encode_simple(OutgoingMessages::RequestFamilyCodes, 1)
}

#[cfg(test)]
mod tests {
    use crate::ToField;

    use super::*;

    #[test]
    fn request_positions() {
        let results = super::request_positions();

        match results {
            Ok(message) => {
                assert_eq!(message[0], OutgoingMessages::RequestPositions.to_field(), "message.type");
                assert_eq!(message[1], "1", "message.version");
            }
            Err(err) => {
                assert!(false, "error encoding request positions: {err}");
            }
        }
    }

    #[test]
    fn cancel_positions() {
        let results = super::cancel_positions();

        match results {
            Ok(message) => {
                assert_eq!(message[0], OutgoingMessages::CancelPositions.to_field(), "message.type");
                assert_eq!(message[1], "1", "message.version");
            }
            Err(err) => {
                assert!(false, "error encoding cancel positions: {err}");
            }
        }
    }

    #[test]
    fn request_family_codes() {
        let results = super::request_family_codes();

        match results {
            Ok(message) => {
                assert_eq!(message[0], OutgoingMessages::RequestFamilyCodes.to_field(), "message.type");
                assert_eq!(message[1], "1", "message.version");
            }
            Err(err) => {
                assert!(false, "error encoding request family codes: {err}");
            }
        }
    }
}
