use fuels::{
    prelude::{abigen, TxParameters, WalletUnlocked},
    tx::{Address, AssetId, Input, Output, Transaction, Receipt},
    contract::script::Script,
    signers::Signer,
};

abigen!(
    LimitOrderStruct,
    "packages/contracts/order-logger/out/debug/order-logger-abi.json"
);

const MIN_GAS: u64 = 100_000;
const TAKE_ORDER_SCRIPT_BINARY: &str = "/Users/walterphillips/BlockchainDev/Sail_v0/packages/contracts/order-script/out/debug/order-script.bin";
pub async fn get_take_order_script() -> Vec<u8> {
    let script_bytecode = std::fs::read(TAKE_ORDER_SCRIPT_BINARY).unwrap();
    script_bytecode
}

async fn build_take_order_tx(
    order: &LimitOrder,
    taker: Address,
    gas_coin: Input,
    predicate_coins_input: Input,
    optional_inputs: &[Input],
    optional_outputs: &[Output],
    params: TxParameters,
) -> Transaction {
    let script_bytecode = get_take_order_script().await;
    // build the tx inputs
    let mut tx_inputs: Vec<Input> = Vec::new();
    tx_inputs.push(predicate_coins_input);
    // tx_inputs.push(gas_coin);
    tx_inputs.append(&mut optional_inputs.to_vec());

    // build the tx outputs 
    let mut tx_outputs: Vec<Output> = Vec::new();
    tx_outputs.push(Output::Coin {
        to: order.maker,
        amount: order.taker_amount,
        asset_id: AssetId::from(order.taker_token.0),
    });
    tx_outputs.push(Output::Coin {
        to: taker,
        amount: order.maker_amount,
        asset_id: AssetId::from(order.taker_token.0),
    });
    tx_outputs.append(&mut optional_outputs.to_vec());

    Transaction::Script {
        gas_price: params.gas_price,
        gas_limit: MIN_GAS,
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

pub async fn inner_take_order(
    order: &LimitOrder,
    wallet: &WalletUnlocked,
    gas_coin: Input,
    predicate_coins_input: Input,
    optional_inputs: &[Input],
    optional_outputs: &[Output],
) -> Vec<Receipt> {
    let mut tx = build_take_order_tx(
        order,
        wallet.address().into(),
        gas_coin,
        predicate_coins_input,
        optional_inputs,
        optional_outputs,
        TxParameters::default(),
    )
    .await;

    sign_and_call_tx(wallet, &mut tx).await
}

async fn sign_and_call_tx(wallet: &WalletUnlocked, tx: &mut Transaction) -> Vec<Receipt> {
    let provider = wallet.get_provider().unwrap();
    wallet.sign_transaction(tx).await.unwrap();
    let script = Script::new(tx.clone());
    script.call(provider).await.unwrap()
}