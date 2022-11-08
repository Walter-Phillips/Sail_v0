library order;


abi OrderSettler {
    fn take(order: LimitOrder);
    fn make(order: LimitOrder);
}

pub struct LimitOrder {
    maker_token: ContractId,  
    taker_token: ContractId,
    maker_amount: u64,
    taker_amount: u64,
    maker: Address,
    salt: u64,
}

