library order;

abi OrderSettler {
    fn take(order: LimitOrder);
    fn make(order: LimitOrder);
}

pub struct LimitOrder {
    maker_token: Address,  
    taker_token: Address,
    maker_amount: u64,
    taker_amount: u64,
    maker: Address,
    salt: u64,
}

