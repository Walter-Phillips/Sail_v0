//using Fuel indexer to log the orders created.
contract;
use fuel_indexer_derive::{graphql_schema, handler};
use fuels_abigen_macro::wasm_abigen;

abi MyContract {
    fn test_function() -> bool;
}

impl MyContract for Contract {
    fn test_function() -> bool {
        true
    }
}
