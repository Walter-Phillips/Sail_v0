use fuels::{
    prelude::{TxParameters},
    tx::{Address, AssetId, Input, Output, Transaction},
};

use crate::utils::build_take_order::*;

const ORDER_SCRIPT_BINARY: &str = "/Users/julian/dev/Sail_v0/packages/contracts/order-script/out/debug/order-script-abi.json";
pub async fn get_order_script() -> Vec<u8> {
    let script_bytecode = std::fs::read(ORDER_SCRIPT_BINARY).unwrap();
    script_bytecode
}

pub async fn build_cancel_order_tx(
    order: &LimitOrder,
    maker: Address,
    gas_coin: Input,
    predicate_coins_input: Input,
    optional_inputs: &[Input],
    optional_outputs: &[Output],
    params: TxParameters,
) -> Transaction {
    let script_bytecode = get_order_script().await;
    // build the tx inputs
    let mut tx_inputs: Vec<Input> = Vec::new();
    tx_inputs.push(predicate_coins_input);
    tx_inputs.push(gas_coin);
    tx_inputs.append(&mut optional_inputs.to_vec());

    // build the tx outputs 
    let mut tx_outputs: Vec<Output> = Vec::new();
    tx_outputs.push(Output::Coin {
        to: maker,
        amount: order.maker_amount,
        asset_id: AssetId::from(order.maker_token.0),
    });
    tx_outputs.append(&mut optional_outputs.to_vec());

    Transaction::Script {
        gas_price: params.gas_price,
        gas_limit: params.gas_limit,
        maturity: params.maturity,
        receipts_root: Default::default(),
        script: script_bytecode,
        script_data: vec![],
        inputs: tx_inputs,
        outputs: tx_outputs,
        witnesses: vec![],
        metadata: None,
    }
}   