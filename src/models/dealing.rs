// use crate::models::{Direction};

// pub struct DealConfirmation {
// 	pub affected_deals: Vec<AffectedDeal>,
// 	pub date: String,
// 	pub deal_id: String,
// 	pub deal_reference: String,
// 	pub deal_status: DealStatus,
// 	pub direction: Direction,
// 	pub epic: String,
// 	pub expiry: String,
// 	pub guaranteed_stop: bool,
// 	pub level: f64,
// 	pub limit_distance: f64,
// 	pub limit_level: f64,
// 	pub profit: f64,
// 	pub profit_currency: String,
// 	pub reason: DealReason,
// 	pub size: f64,
// 	pub status: PositionStatus,
// 	pub stop_distance: f64,
// 	pub stop_level: f64,
// 	pub trailing_stop: bool
// }

// pub struct AffectedDeal {
// 	pub deal_id: String,
// 	pub status: AffectedDealStatus
// }

// pub enum AffectedDealStatus {
// 	Amended,
// 	Deleted,
// 	FullyClosed,
// 	Opened,
// 	PartiallyOpened
// }

// pub enum DealStatus {
// 	Accepted,
// 	Rejected
// }

// pub enum DealReason {
// 	AccountNotEnabledToTrading,
// 	AttachedOrderLevelError,
// 	AttachedOrderTrailingStopError,
// 	CannotChangeStopType,
// 	CannotRemoveStop,
// 	ClosingOnlyTradesAcceptedOnThisMarket,
// 	ClosingsOnlyAccount,
// 	ConflictingOrder,
// 	ContactSupportInstrumentError,
// 	CrSpacing,
// 	DuplicateOrderError,
// 	ExchangeManualOverride,
// 	ExpiryLessThanSprintMarketMinExpiry,
// 	FinanceRepeatDealing,
// 	ForceOpenOnSameMarketDifferentCurrency,
// 	GeneralError,
// 	GoodTillDateInThePast,
// 	InstrumentNotFound,
// 	InstrumentNotTradeableInThisCurrency,
// 	InsufficientFunds,
// 	LevelToleranceError,
// 	LimitOrderWrongSideOfMarket,
// 	ManualOrderTimeout,
// 	MarginError,
// 	MarketClosed,
// 	MarketClosedWithEdits,
// 	MarketClosing,
// 	MarketNotBorrowable,
// 	MarketOffline,
// 	MarketOrdersNotAllowedOnInstrument,
// 	MarketPhoneOnly,
// 	MarketRolled,
// 	MarketUnavailableToClient,
// 	MaxAutoSizeExceeded,
// 	MinimumOrderSizeError,
// 	MoveAwayOnlyLimit,
// 	MoveAwayOnlyStop,
// 	MoveAwayOnlyTriggerLevel,
// 	NcrPositionsOnCrAccount,
// 	OpposingDirectionOrdersNotAllowed,
// 	OpposingPositionsNotAllowed,
// 	OrderDeclined,
// 	OrderLocked,
// 	OrderNotFound,
// 	OrderSizeCannotBeFilled,
// 	OverNormalMarketSize,
// 	PartiallyClosedPositionNotDeleted,
// 	PositionAlreadyExistsInOppositeDirection,
// 	PositionNotAvailableToCancel,
// 	PositionNotAvailableToClose,
// 	PositionNotFound,
// 	RejectCfdOrderOnSpreadbetAccount,
// 	RejectSpreadbetOrderOnCfdAccount,
// 	SizeIncrement,
// 	SprintMarketExpiryAfterMarketClose,
// 	StopOrLimitNotAllowed,
// 	StopRequiredError,
// 	StrikeLevelTolerance,
// 	Success,
// 	TrailingStopNotAllowed,
// 	Unknown,
// 	WrongSideOfMarket
// }

// pub enum PositionStatus {
// 	Amended,
// 	Closed,
// 	Deleted,
// 	Open,
// 	PartiallyClosed
// }

// pub struct Positions {
// 	pub positions: Vec<Position>
// }

// pub struct Position {
// 	pub market: MarketData,
// 	pub position: PositionData
// }

// pub struct MarketData {
// 	pub bid: f64,
// 	pub delay_time: f64,
// 	pub epic: String,
// 	pub expiry: String,
// 	pub high: f64,
// 	pub instrument_name: String,
// 	pub instrument_type: InstrumentType,
// 	pub lot_size: f64,
// 	pub low: f64,
// 	pub market_status: MarketStatus,
// 	pub net_change: f64,
// 	pub offer: f64,
// 	pub percentage_change: f64,
// 	pub scaling_factor: f64,
// 	pub streaming_prices_available: bool,
// 	pub update_time: String,
// 	pub update_time_utc: String
// }

// pub enum InstrumentType {
// 	Binary,
// 	BungeeCapped,
// 	BungeeCommodities,
// 	BungeeCurrencies,
// 	BungeeIndices,
// 	Commodities,
// 	Currencies,
// 	Indices,
// 	KnockoutsCommodities,
// 	KnockoutsCurrencies,
// 	KnockoutsIndices,
// 	KnockoutsShares,
// 	OptCommodities,
// 	OptCurrencies,
// 	OptIndices,
// 	OptRates,
// 	OptShares,
// 	Rates,
// 	Sectors,
// 	Shares,
// 	SprintMarket,
// 	TestMarket,
// 	Unknown
// }

// pub enum MarketStatus {
// 	Closed,
// 	EditsOnly,
// 	Offline,
// 	OnAuction,
// 	OnAuctionNoEdits,
// 	Suspended,
// 	Tradeable
// }

// pub struct PositionData {
// 	pub contract_size: f64,
// 	pub controlled_risk: bool,
// 	pub created_date: String,
// 	pub created_date_utc: String,
// 	pub currency: String,
// 	pub deal_id: String,
// 	pub deal_reference: String,
// 	pub direction: Direction,
// 	pub level: f64,
// 	pub limit_level: f64,
// 	pub limited_risk_premium: f64,
// 	pub size: f64,
// 	pub stop_level: f64,
// 	pub trailing_step: f64,
// 	pub trailing_stop_distance: f64
// }

// pub struct ClosePosition {
// 	pub deal_id: Option<String>,
// 	pub direction: Option<Direction>,
// 	pub epic: Option<String>,
// 	pub expiry: Option<String>,
// 	pub level: Option<f64>,
// 	pub order_type: Option<OrderType>,
// 	pub quote_id: Option<String>,
// 	pub size: Option<f64>,
// 	pub time_in_force: Option<TimeInForce>
// }

// pub enum OrderType {
// 	Limit,
// 	Market,
// 	Quote
// }

// pub enum TimeInForce {
// 	ExecuteAndEliminate,
// 	FillOrKill
// }

// pub struct DealRef {
// 	pub deal_reference: String
// }

// pub struct CreatePosition {
// 	pub currency_code: Option<String>,
// 	pub deal_reference: Option<String>,
// 	pub direction: Option<Direction>,
// 	pub epic: Option<String>,
// 	pub expiry: Option<String>,
// 	pub force_open: Option<bool>,
// 	pub guaranteed_stop: Option<bool>,
// 	pub level: Option<f64>,
// 	pub limit_distance: Option<f64>,
// 	pub limit_level: Option<f64>,
// 	pub order_type: Option<OrderType>,
// 	pub quote_id: Option<String>,
// 	pub size: Option<f64>,
// 	pub stop_distance: Option<f64>,
// 	pub stop_level: Option<f64>,
// 	pub time_in_force: Option<TimeInForce>,
// 	pub trailing_stop: Option<bool>,
// 	pub trailing_stop_increment: Option<f64>
// }

// pub struct UpdatePosition {
// 	pub guaranteed_stop: Option<bool>,
// 	pub limit_level: Option<f64>,
// 	pub stop_level: Option<f64>,
// 	pub trailing_stop: Option<bool>,
// 	pub trailing_stop_distance: Option<f64>,
// 	pub trailing_stop_increment: Option<f64>
// }

// pub struct SprintMarketPositions {
// 	pub sprint_market_positions: Vec<SprintMarketPosition>
// }

// pub struct SprintMarketPosition {
// 	pub created_date: String,
// 	pub currency: String,
// 	pub deal_id: String,
// 	pub description: String,
// 	pub direction: Direction,
// 	pub epic: String,
// 	pub expiry_time: String,
// 	pub instrument_name: String,
// 	pub market_status: MarketStatus,
// 	pub payout_amount: f64,
// 	pub size: f64,
// 	pub strike_level: f64
// }

// pub struct CreateSprintMarketPosition {
// 	pub deal_reference: String,
// 	pub direction: Direction,
// 	pub epic: String,
// 	pub expiry_period: SprintMarketExpiryPeriod,
// 	pub size: f64
// }

// pub enum SprintMarketExpiryPeriod {
// 	FiveMinutes,
// 	OneMinute,
// 	SixtyMinutes,
// 	TwentyMinutes,
// 	TwoMinutes
// }

// pub struct WorkingOrders {
// 	pub working_orders: Vec<WorkingOrder>
// }

// pub struct WorkingOrder {
// 	market_data: MarketData2,
// 	working_order_data: WorkingOrderData
// }

// pub struct MarketData2 {
// 	pub bid: f64,
// 	pub delay_time: f64,
// 	pub epic: String,
// 	pub exchange_id: String,
// 	pub expiry: String,
// 	pub high: f64,
// 	pub instrument_name: String,
// 	pub instrument_type: InstrumentType,
// 	pub lot_size: f64,
// 	pub low: f64,
// 	pub market_status: MarketStatus,
// 	pub net_change: f64,
// 	pub offer: f64,
// 	pub percentage_change: f64,
// 	pub scaling_factor: f64,
// 	pub streaming_prices_enabled: bool,
// 	pub update_time: String,
// 	pub update_time_utc: String
// }

// pub struct WorkingOrderData {
// 	pub created_date: String,
// 	pub created_date_utc: String,
// 	pub currency_code: String,
// 	pub deal_id: String,
// 	pub direction: Direction,
// 	pub dma: bool,
// 	pub epic: String,
// 	pub good_till_date: String,
// 	pub good_till_date_iso: String,
// 	pub guaranteed_stop: bool,
// 	pub limit_distance: f64,
// 	pub limited_risk_premium: f64,
// 	pub order_level: f64,
// 	pub order_size: f64,
// 	pub order_type: WorkingOrderType,
// 	pub stop_distance: f64,
// 	pub time_in_force: WorkingOrderTimeInForce
// }

// pub enum WorkingOrderType {
// 	Limit,
// 	Stop
// }

// pub enum WorkingOrderTimeInForce {
// 	GoodTillCancelled,
// 	GoodTillDate
// }

// pub struct CreateWorkingOrder {
// 	pub currency_code: Option<String>,
// 	pub deal_reference: Option<String>,
// 	pub direction: Option<Direction>,
// 	pub epic: Option<String>,
// 	pub expiry: Option<String>,
// 	pub force_open: Option<bool>,
// 	pub good_till_date: Option<String>,
// 	pub guaranteed_stop: Option<bool>,
// 	pub level: Option<f64>,
// 	pub limit_distance: Option<f64>,
// 	pub limit_level: Option<f64>,
// 	pub size: Option<f64>,
// 	pub stop_distance: Option<f64>,
// 	pub stop_level: Option<f64>,
// 	pub time_in_force: Option<WorkingOrderTimeInForce>,
// 	pub r#type: Option<WorkingOrderType>
// }

// pub struct UpdateWorkingOrder {
// 	pub good_till_date: Option<String>,
// 	pub guaranteed_stop: Option<bool>,
// 	pub level: Option<f64>,
// 	pub limit_distance: Option<f64>,
// 	pub limit_level: Option<f64>,
// 	pub stop_distance: Option<f64>,
// 	pub stop_level: Option<f64>,
// 	pub time_in_force: Option<WorkingOrderTimeInForce>,
// 	pub r#type: Option<WorkingOrderType>
// }