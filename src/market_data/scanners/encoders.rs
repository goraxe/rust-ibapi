use std::iter::Scan;

use crate::{messages::OutgoingMessages, server_versions};
//use crate::market_data::ScannerSubscription;
use super::*;

pub(crate) fn scanner_subscription(
    server_version: i32,
    request_id: i32,
    subscription: &ScannerSubscription,
    scanner_filter_options: &ScannerSubscriptionFilter,
    subscription_options: &ScannerSubscriptionOptions,
) -> Result<RequestMessage, Error> {
    const VERSION: i32 = 4;

    let mut message = RequestMessage::default();

    message.push_field(&OutgoingMessages::RequestScannerSubscription);

    if server_version < server_versions::SCANNER_GENERIC_OPTS {
        message.push_field(&VERSION);
    }

    message.push_field(&request_id);
    if subscription.number_of_rows == ScannerSubscription::default().number_of_rows {
        message.push_field(&"");
    } else {
        message.push_field(&subscription.number_of_rows);
    }
    message.push_field(&subscription.instrument);
    message.push_field(&subscription.location_code);
    message.push_field(&subscription.scan_code);
    if subscription.above_price == ScannerSubscription::default().above_price {
        message.push_field(&"");
    } else {
        message.push_field(&subscription.above_price);
    }
    if subscription.below_price == ScannerSubscription::default().below_price {
        message.push_field(&"");
    } else {
        message.push_field(&subscription.below_price);
    }
    if subscription.above_volume == ScannerSubscription::default().above_volume {
        message.push_field(&"");
    } else {
        message.push_field(&subscription.above_volume);
    }
    if subscription.market_cap_above == ScannerSubscription::default().market_cap_above {
        message.push_field(&"");
    } else {
        message.push_field(&subscription.market_cap_above);
    }

    if subscription.market_cap_below == ScannerSubscription::default().market_cap_below {
        message.push_field(&"");
    } else {
        message.push_field(&subscription.market_cap_below);
    }
    message.push_field(&subscription.moody_rating_above);
    message.push_field(&subscription.moody_rating_below);
    message.push_field(&subscription.sp_rating_above);
    message.push_field(&subscription.sp_rating_below);
    message.push_field(&subscription.maturity_date_above);
    message.push_field(&subscription.maturity_date_below);
    if subscription.coupon_rate_above == ScannerSubscription::default().coupon_rate_above {
        message.push_field(&"");
    } else {
        message.push_field(&subscription.coupon_rate_above);
    }
    if subscription.coupon_rate_below == ScannerSubscription::default().coupon_rate_above {
        message.push_field(&"");
    } else {
        message.push_field(&subscription.coupon_rate_below);
    }
    if subscription.exclude_converiable == ScannerSubscription::default().exclude_converiable {
        message.push_field(&"");
    } else {
        message.push_field(&subscription.exclude_converiable);
    }
    if subscription.average_options_volume_above == ScannerSubscription::default().average_options_volume_above {
        message.push_field(&"");
    } else {
        message.push_field(&subscription.average_options_volume_above);
    }
    message.push_field(&subscription.scanner_settings_pairs);
    message.push_field(&subscription.stock_type_filter);

    if server_version >= server_versions::SCANNER_GENERIC_OPTS {
        message.push_field(scanner_filter_options);
    }

    if server_version >= server_versions::LINKING {
        message.push_field(subscription_options);
    }

    Ok(message)
}

pub(crate) fn cancel_scanner_subscription(request_id: i32) -> Result<RequestMessage, Error> {
    const VERSION: i32 = 1;

    let mut message = RequestMessage::default();

    message.push_field(&OutgoingMessages::CancelScannerSubscription);
    message.push_field(&VERSION);
    message.push_field(&request_id);
    Ok(message)
}
