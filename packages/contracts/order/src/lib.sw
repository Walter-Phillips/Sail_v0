library order;
abi OrderSettler {
    fn take(order: LimitOrder);
    fn make(order: LimitOrder);
}

pub struct LimitOrder {
    maker_token: b256, // using b256 for convenience, may be a bad idea. 
    taker_token: b256,
    maker_amount: u64,
    taker_amount: u64,
    maker: Address,
    salt: u64, // arbitrary salt for uniqueness in order hash
}

