#[derive(Debug)]
pub enum IncomingMessage {
    NotValid = -1,
    TickPrice = 1,
    TickSize = 2,
    OrderStatus = 3,
    Error = 4,
    OpenOrder = 5,
    AccountValue = 6,
    PortfolioValue = 7,
    AccountUpdateTime = 8,
    NextValidId = 9,
    ContractData = 10,
    ExecutionData = 11,
    MarketDepth = 12,
    MarketDepthL2 = 13,
    NewsBulletins = 14,
    ManagedAccounts = 15,
    ReceiveFA = 16,
    HistoricalData = 17,
    BondContractData = 18,
    ScannerParameters = 19,
    ScannerData = 20,
    TickOptionComputation = 21,
    TickGeneric = 45,
    Tickstring = 46,
    TickEFP = 47, //TICK EFP 47
    CurrentTime = 49,
    RealTimeBars = 50,
    FundamentalData = 51,
    ContractDataEnd = 52,
    OpenOrderEnd = 53,
    AccountDownloadEnd = 54,
    ExecutionDataEnd = 55,
    DeltaNeutralValidation = 56,
    TickSnapshotEnd = 57,
    MarketDataType = 58,
    CommissionsReport = 59,
    Position = 61,
    PositionEnd = 62,
    AccountSummary = 63,
    AccountSummaryEnd = 64,
    VerifyMessageApi = 65,
    VerifyCompleted = 66,
    DisplayGroupList = 67,
    DisplayGroupUpdated = 68,
    VerifyAndAuthMessageApi = 69,
    VerifyAndAuthCompleted = 70,
    PositionMulti = 71,
    PositionMultiEnd = 72,
    AccountUpdateMulti = 73,
    AccountUpdateMultiEnd = 74,
    SecurityDefinitionOptionParameter = 75,
    SecurityDefinitionOptionParameterEnd = 76,
    SoftDollarTier = 77,
    FamilyCodes = 78,
    SymbolSamples = 79,
    MktDepthExchanges = 80,
    TickReqParams = 81,
    SmartComponents = 82,
    NewsArticle = 83,
    TickNews = 84,
    NewsProviders = 85,
    HistoricalNews = 86,
    HistoricalNewsEnd = 87,
    HeadTimestamp = 88,
    HistogramData = 89,
    HistoricalDataUpdate = 90,
    RerouteMktDataReq = 91,
    RerouteMktDepthReq = 92,
    MarketRule = 93,
    PnL = 94,
    PnLSingle = 95,
    HistoricalTick = 96,
    HistoricalTickBidAsk = 97,
    HistoricalTickLast = 98,
    TickByTick = 99,
    OrderBound = 100,
    CompletedOrder = 101,
    CompletedOrdersEnd = 102,
    ReplaceFAEnd = 103,
    WshMetaData = 104,
    WshEventData = 105,
    HistoricalSchedule = 106,
    UserInfo = 107,
}

impl From<i32> for IncomingMessage {
    fn from(value: i32) -> IncomingMessage {
        match value {
            1 => IncomingMessage::TickPrice,
            2 => IncomingMessage::TickSize,
            3 => IncomingMessage::OrderStatus,
            4 => IncomingMessage::Error,
            5 => IncomingMessage::OpenOrder,
            6 => IncomingMessage::AccountValue,
            7 => IncomingMessage::PortfolioValue,
            8 => IncomingMessage::AccountUpdateTime,
            9 => IncomingMessage::NextValidId,
            10 => IncomingMessage::ContractData,
            11 => IncomingMessage::ExecutionData,
            12 => IncomingMessage::MarketDepth,
            13 => IncomingMessage::MarketDepthL2,
            14 => IncomingMessage::NewsBulletins,
            15 => IncomingMessage::ManagedAccounts,
            16 => IncomingMessage::ReceiveFA,
            17 => IncomingMessage::HistoricalData,
            18 => IncomingMessage::BondContractData,
            19 => IncomingMessage::ScannerParameters,
            20 => IncomingMessage::ScannerData,
            21 => IncomingMessage::TickOptionComputation,
            45 => IncomingMessage::TickGeneric,
            46 => IncomingMessage::Tickstring,
            47 => IncomingMessage::TickEFP, //TICK EFP 47
            49 => IncomingMessage::CurrentTime,
            50 => IncomingMessage::RealTimeBars,
            51 => IncomingMessage::FundamentalData,
            52 => IncomingMessage::ContractDataEnd,
            53 => IncomingMessage::OpenOrderEnd,
            54 => IncomingMessage::AccountDownloadEnd,
            55 => IncomingMessage::ExecutionDataEnd,
            56 => IncomingMessage::DeltaNeutralValidation,
            57 => IncomingMessage::TickSnapshotEnd,
            58 => IncomingMessage::MarketDataType,
            59 => IncomingMessage::CommissionsReport,
            // IncomingMessage::Position = 61,
            // IncomingMessage::PositionEnd = 62,
            // IncomingMessage::AccountSummary = 63,
            // IncomingMessage::AccountSummaryEnd = 64,
            // IncomingMessage::VerifyMessageApi = 65,
            // IncomingMessage::VerifyCompleted = 66,
            // IncomingMessage::DisplayGroupList = 67,
            // IncomingMessage::DisplayGroupUpdated = 68,
            // IncomingMessage::VerifyAndAuthMessageApi = 69,
            // IncomingMessage::VerifyAndAuthCompleted = 70,
            // IncomingMessage::PositionMulti = 71,
            // IncomingMessage::PositionMultiEnd = 72,
            // IncomingMessage::AccountUpdateMulti = 73,
            // IncomingMessage::AccountUpdateMultiEnd = 74,
            // IncomingMessage::SecurityDefinitionOptionParameter = 75,
            // IncomingMessage::SecurityDefinitionOptionParameterEnd = 76,
            // IncomingMessage::SoftDollarTier = 77,
            // IncomingMessage::FamilyCodes = 78,
            // IncomingMessage::SymbolSamples = 79,
            // IncomingMessage::MktDepthExchanges = 80,
            // IncomingMessage::TickReqParams = 81,
            // IncomingMessage::SmartComponents = 82,
            // IncomingMessage::NewsArticle = 83,
            // IncomingMessage::TickNews = 84,
            // IncomingMessage::NewsProviders = 85,
            // IncomingMessage::HistoricalNews = 86,
            // IncomingMessage::HistoricalNewsEnd = 87,
            // IncomingMessage::HeadTimestamp = 88,
            // IncomingMessage::HistogramData = 89,
            // IncomingMessage::HistoricalDataUpdate = 90,
            // IncomingMessage::RerouteMktDataReq = 91,
            // IncomingMessage::RerouteMktDepthReq = 92,
            // IncomingMessage::MarketRule = 93,
            // IncomingMessage::PnL = 94,
            // IncomingMessage::PnLSingle = 95,
            // IncomingMessage::HistoricalTick = 96,
            // IncomingMessage::HistoricalTickBidAsk = 97,
            // IncomingMessage::HistoricalTickLast = 98,
            // IncomingMessage::TickByTick = 99,
            // IncomingMessage::OrderBound = 100,
            // IncomingMessage::CompletedOrder = 101,
            // IncomingMessage::CompletedOrdersEnd = 102,
            // IncomingMessage::ReplaceFAEnd = 103,
            // IncomingMessage::WshMetaData = 104,
            // IncomingMessage::WshEventData = 105,
            // IncomingMessage::HistoricalSchedule = 106,
            // IncomingMessage::UserInfo = 107,
            _ => IncomingMessage::NotValid,
        }
    }
}

#[derive(Debug)]
pub enum OutgoingMessage {
    RequestMarketData = 1,
    CancelMarketData = 2,
    PlaceOrder = 3,
    CancelOrder = 4,
    RequestOpenOrders = 5,
    RequestAccountData = 6,
    RequestExecutions = 7,
    RequestIds = 8,
    RequestContractData = 9,
    RequestMarketDepth = 10,
    CancelMarketDepth = 11,
    RequestNewsBulletins = 12,
    CancelNewsBulletin = 13,
    ChangeServerLog = 14,
    RequestAutoOpenOrders = 15,
    RequestAllOpenOrders = 16,
    RequestManagedAccounts = 17,
    RequestFA = 18,
    ReplaceFA = 19,
    RequestHistoricalData = 20,
    ExerciseOptions = 21,
    RequestScannerSubscription = 22,
    CancelScannerSubscription = 23,
    RequestScannerParameters = 24,
    CancelHistoricalData = 25,
    RequestCurrentTime = 49,
    RequestRealTimeBars = 50,
    CancelRealTimeBars = 51,
    RequestFundamentalData = 52,
    CancelFundamentalData = 53,
    ReqCalcImpliedVolat = 54,
    ReqCalcOptionPrice = 55,
    CancelImpliedVolatility = 56,
    CancelOptionPrice = 57,
    RequestGlobalCancel = 58,
    RequestMarketDataType = 59,
    RequestPositions = 61,
    RequestAccountSummary = 62,
    CancelAccountSummary = 63,
    CancelPositions = 64,
    VerifyRequest = 65,
    VerifyMessage = 66,
    QueryDisplayGroups = 67,
    SubscribeToGroupEvents = 68,
    UpdateDisplayGroup = 69,
    UnsubscribeFromGroupEvents = 70,
    StartApi = 71,
    VerifyAndAuthRequest = 72,
    VerifyAndAuthMessage = 73,
    RequestPositionsMulti = 74,
    CancelPositionsMulti = 75,
    RequestAccountUpdatesMulti = 76,
    CancelAccountUpdatesMulti = 77,
    RequestSecurityDefinitionOptionalParameters = 78,
    RequestSoftDollarTiers = 79,
    RequestFamilyCodes = 80,
    RequestMatchingSymbols = 81,
    RequestMktDepthExchanges = 82,
    RequestSmartComponents = 83,
    RequestNewsArticle = 84,
    RequestNewsProviders = 85,
    RequestHistoricalNews = 86,
    RequestHeadTimestamp = 87,
    RequestHistogramData = 88,
    CancelHistogramData = 89,
    CancelHeadTimestamp = 90,
    RequestMarketRule = 91,
    ReqPnL = 92,
    CancelPnL = 93,
    ReqPnLSingle = 94,
    CancelPnLSingle = 95,
    ReqHistoricalTicks = 96,
    ReqTickByTickData = 97,
    CancelTickByTickData = 98,
    ReqCompletedOrders = 99,
    ReqWshMetaData = 100,
    CancelWshMetaData = 101,
    ReqWshEventData = 102,
    CancelWshEventData = 103,
    ReqUserInfo = 104,
}
