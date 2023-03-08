library order;


abi OrderSettler {
    fn take(order: LimitOrder);
    fn make(order: LimitOrder);
    fn cancel(order: LimitOrder);
    fn update(order: LimitOrder);
}

pub struct LimitOrder {
    maker_token: b256,  
    taker_token: b256,
    maker_amount: u64,
    taker_amount: u64,
    maker: Address,
    salt: u64,
}

