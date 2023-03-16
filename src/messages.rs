use crate::ToField;

#[derive(Debug, PartialEq)]
pub enum IncomingMessages {
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

impl From<i32> for IncomingMessages {
    fn from(value: i32) -> IncomingMessages {
        match value {
            1 => IncomingMessages::TickPrice,
            2 => IncomingMessages::TickSize,
            3 => IncomingMessages::OrderStatus,
            4 => IncomingMessages::Error,
            5 => IncomingMessages::OpenOrder,
            6 => IncomingMessages::AccountValue,
            7 => IncomingMessages::PortfolioValue,
            8 => IncomingMessages::AccountUpdateTime,
            9 => IncomingMessages::NextValidId,
            10 => IncomingMessages::ContractData,
            11 => IncomingMessages::ExecutionData,
            12 => IncomingMessages::MarketDepth,
            13 => IncomingMessages::MarketDepthL2,
            14 => IncomingMessages::NewsBulletins,
            15 => IncomingMessages::ManagedAccounts,
            16 => IncomingMessages::ReceiveFA,
            17 => IncomingMessages::HistoricalData,
            18 => IncomingMessages::BondContractData,
            19 => IncomingMessages::ScannerParameters,
            20 => IncomingMessages::ScannerData,
            21 => IncomingMessages::TickOptionComputation,
            45 => IncomingMessages::TickGeneric,
            46 => IncomingMessages::Tickstring,
            47 => IncomingMessages::TickEFP, //TICK EFP 47
            49 => IncomingMessages::CurrentTime,
            50 => IncomingMessages::RealTimeBars,
            51 => IncomingMessages::FundamentalData,
            52 => IncomingMessages::ContractDataEnd,
            53 => IncomingMessages::OpenOrderEnd,
            54 => IncomingMessages::AccountDownloadEnd,
            55 => IncomingMessages::ExecutionDataEnd,
            56 => IncomingMessages::DeltaNeutralValidation,
            57 => IncomingMessages::TickSnapshotEnd,
            58 => IncomingMessages::MarketDataType,
            59 => IncomingMessages::CommissionsReport,
            61 => IncomingMessages::Position,
            62 => IncomingMessages::PositionEnd,
            63 => IncomingMessages::AccountSummary,
            64 => IncomingMessages::AccountSummaryEnd,
            65 => IncomingMessages::VerifyMessageApi,
            66 => IncomingMessages::VerifyCompleted,
            67 => IncomingMessages::DisplayGroupList,
            68 => IncomingMessages::DisplayGroupUpdated,
            69 => IncomingMessages::VerifyAndAuthMessageApi,
            70 => IncomingMessages::VerifyAndAuthCompleted,
            71 => IncomingMessages::PositionMulti,
            72 => IncomingMessages::PositionMultiEnd,
            73 => IncomingMessages::AccountUpdateMulti,
            74 => IncomingMessages::AccountUpdateMultiEnd,
            75 => IncomingMessages::SecurityDefinitionOptionParameter,
            76 => IncomingMessages::SecurityDefinitionOptionParameterEnd,
            77 => IncomingMessages::SoftDollarTier,
            78 => IncomingMessages::FamilyCodes,
            79 => IncomingMessages::SymbolSamples,
            80 => IncomingMessages::MktDepthExchanges,
            81 => IncomingMessages::TickReqParams,
            82 => IncomingMessages::SmartComponents,
            83 => IncomingMessages::NewsArticle,
            84 => IncomingMessages::TickNews,
            85 => IncomingMessages::NewsProviders,
            86 => IncomingMessages::HistoricalNews,
            87 => IncomingMessages::HistoricalNewsEnd,
            88 => IncomingMessages::HeadTimestamp,
            89 => IncomingMessages::HistogramData,
            90 => IncomingMessages::HistoricalDataUpdate,
            91 => IncomingMessages::RerouteMktDataReq,
            92 => IncomingMessages::RerouteMktDepthReq,
            93 => IncomingMessages::MarketRule,
            94 => IncomingMessages::PnL,
            95 => IncomingMessages::PnLSingle,
            96 => IncomingMessages::HistoricalTick,
            97 => IncomingMessages::HistoricalTickBidAsk,
            98 => IncomingMessages::HistoricalTickLast,
            99 => IncomingMessages::TickByTick,
            100 => IncomingMessages::OrderBound,
            101 => IncomingMessages::CompletedOrder,
            102 => IncomingMessages::CompletedOrdersEnd,
            103 => IncomingMessages::ReplaceFAEnd,
            104 => IncomingMessages::WshMetaData,
            105 => IncomingMessages::WshEventData,
            106 => IncomingMessages::HistoricalSchedule,
            107 => IncomingMessages::UserInfo,
            _ => IncomingMessages::NotValid,
        }
    }
}

pub fn order_id_index(kind: IncomingMessages) -> Option<usize> {
    match kind {
        IncomingMessages::OpenOrder | IncomingMessages::OrderStatus => Some(1),
        IncomingMessages::ExecutionData | IncomingMessages::ExecutionDataEnd => Some(2),
        _ => None,
    }
}

pub fn request_id_index(kind: IncomingMessages) -> Option<usize> {
    match kind {
        IncomingMessages::ContractData
        | IncomingMessages::TickByTick
        | IncomingMessages::SymbolSamples
        | IncomingMessages::OpenOrder
        | IncomingMessages::ExecutionData => Some(1),
        IncomingMessages::ContractDataEnd | IncomingMessages::RealTimeBars | IncomingMessages::Error | IncomingMessages::ExecutionDataEnd => Some(2),
        _ => None,
    }
}

#[derive(Debug, Copy, Clone)]
pub enum OutgoingMessages {
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

impl ToField for OutgoingMessages {
    fn to_field(&self) -> String {
        (*self as i32).to_string()
    }
}
