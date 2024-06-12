use log::trace;

use crate::{contracts::{self, Contract, ContractDetails}, messages::ResponseMessage, Error};

use super::ScannerData;



pub(crate) fn decode_scan_data_list(message: &mut ResponseMessage ) -> Result<Vec<ScannerData>, Error> {

    message.skip(); // version
    message.skip(); // request id 

    trace!("parsing num_items");
    let num_items = message.next_int()?;

    let mut results = Vec::<ScannerData>::new();

    for _ in 0..num_items-1 {
        trace!("parsing rank");
        let rank = message.next_int()?;
        trace!("parsing contract");
        let contract_id = message.next_long()?;
        trace!("parsing symbol");
        let contract_contract_symbol = message.next_string()?;
        trace!("parsing sec type");
        let contract_contract_sectype = message.next_string()?;
        trace!("parsing last trade");
        let contract_contract_last_trade = message.next_string()?;
        trace!("parsing contract strike");
        let contract_contract_strike = message.next_double()?;
        trace!("parsing contract right");
        let contract_contract_right = message.next_string()?;
        trace!("parsing contrat exchange");
        let contract_contract_exchange = message.next_string()?;
        trace!("parsing contract currency");
        let contract_contract_currency = message.next_string()?;
        trace!("parsing contract symbol");
        let contract_contract_local_symbol = message.next_string()?;
        trace!("parsing contract market");
        let contract_market_name = message.next_string()?;
        trace!("parsing contract trading class");
        let contract_contract_trading_class = message.next_string()?;
        trace!("parsing distance");
        let distance = message.next_string()?;
        trace!("parsing benchmark");
        let benchmark = message.next_string()?;
        trace!("parsing projection");
        let projection = message.next_string()?;
        trace!("parsing legs");
        let legs = message.next_string()?;

        let data = ScannerData{
            rank,
            contract: ContractDetails{
                contract: contracts::Contract {
                    contract_id: contract_id.try_into().unwrap(),
                    symbol: contract_contract_symbol,
                    //security_type:contract_contract_sectype, 
                    last_trade_date_or_contract_month: contract_contract_last_trade,
                    strike: contract_contract_strike,
                //    right: contract_contract_right, 
                    exchange: contract_contract_exchange,
                    currency: contract_contract_currency,
                    local_symbol: contract_contract_local_symbol,
                    trading_class: contract_contract_trading_class,
                    security_id_type: contract_contract_sectype,
                    ..Contract::default()
                },
                market_name: contract_market_name,
                ..ContractDetails::default()

            },
            distance,
            benchmark,
            projection,
            legs
        };
       results.push(data);

    }

        Ok(results)
}
