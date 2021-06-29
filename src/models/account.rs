pub struct Account {
	account_alias: String,
	account_id: String,
	account_name: String,
	account_type: AccountType,
	balance: Balance,
	can_transfer_from: bool,
	can_transfer_to: bool,
	currency: String,
	preferred: bool,
	status: AccountStatus
}

pub enum AccountType {
	CFD,
	Physical,
	SpreadBet
}

pub struct Balance {
	available: f64,
	balance: f64,
	deposit: f64,
	profit_loss: f64
}

pub enum AccountStatus {
	Disabled,
	Enabled,
	SuspendedFromDealing
}

pub struct Preferences {
	trailing_stops_enabled: bool
}

pub struct History {
	activities: Vec<Activity>,
	metadata: Metadata
}

pub struct History2 {
	activities: Vec<Activity2>
}

pub struct Activity {
	channel: Channel,
	date: String,
	deal_id: String,
	description: String,
	details: ActivityDetails,
	epic: String,
	period: String,
	status: ActivityStatus,
	r#type: ActivityType
}

pub struct Activity2 {
	action_status: String,
	activity: String,
	activity_history_id: String,
	channel: String,
	currency: String,
	date: String,
	deal_id: String,
	epic: String,
	level: String,
	limit: String,
	market_name: String,
	period: String,
	result: String,
	size: String,
	stop: String,
	stop_type: String,
	time: String
}

pub enum Channel {
	Dealer,
	Mobile,
	PublicFixApi,
	PublicWebApi,
	System,
	Web
}

pub struct ActivityDetails {
	actions: Vec<ActivityAction>,
	currency: String,
	deal_reference: String,
	direction: Direction,
	good_till_date: String,
	guaranteed_stop: bool,
	level: f64,
	limit_distance: f64,
	limit_level: f64,
	market_name: String,
	size: f64,
	stop_distance: f64,
	stop_level: f64,
	trailing_step: f64,
	trailing_stop_distance: f64
}

pub struct ActivityAction {
	action_type: ActivityActionType,
	affected_deal_id: String
}

pub enum ActivityActionType {
	LimitOrderAmended,
	LimitOrderDeleted,
	LimitOrderFilled,
	LimitOrderOpened,
	LimitOrderRolled,
	PositionClosed,
	PositionDeleted,
	PositionOpened,
	PositionPartiallyClosed,
	PositionRolled,
	StopLimitAmended,
	StopOrderAmended,
	StopOrderDeleted,
	StopOrderFilled,
	StopOrderOpened,
	StopOrderRolled,
	Unknown,
	WorkingOrderDeleted
}

pub enum Direction {
	Buy,
	Sell
}

pub enum ActivityStatus {
	Accepted,
	Rejected,
	Unknown
}

pub enum ActivityType {
	EditStopAndLimit,
	Position,
	System,
	WorkingOrder
}

pub struct Metadata {
	pading: Paging
}

pub struct Paging {
	next: String,
	size: u32
}