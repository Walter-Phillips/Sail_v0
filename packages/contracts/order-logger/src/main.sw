contract;
use std::logging::log;
use order::{LimitOrder, OrderSettler};

struct MakeOrder {
    order: LimitOrder,
}
struct TakeOrder {
    order: LimitOrder,
}
struct CancelOrder {
    order: LimitOrder,
}
struct UpdateOrder {
    order: LimitOrder,
}

impl OrderSettler for Contract {
    fn take(order: LimitOrder) {
        log(TakeOrder { order })
    }
    fn make(order: LimitOrder) {
        log(MakeOrder { order })
    }
    fn cancel(order: LimitOrder) {
        log(CancelOrder { order })
    }
    fn update(order: LimitOrder) {
        log(UpdateOrder { order })
    }
}

fn deposit() {
    // ...
}

fn withdraw() {
    // ...
}