extern crate alloc;
use fuel_indexer_macros::{graphql_schema, handler};
use fuels_abigen_macro::wasm_abigen;

#[handler]
fn handle_init(event: InitEvent) {
    Logger::info("Make Created");
    let InitEvent {
        order
    } = event;

    let mut makeOrder = match Pool::load(contract_id) {
        Some(t) => t,
        None => LimitOrder {
            order,
        },
    };

    makeOrder.save();
}