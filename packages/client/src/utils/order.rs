//create the make order and the take order

use crate::utils::build_take_order::*;
use crate::utils::create_predicate::*;

use fuels::{
    contract::predicate::Predicate,
    prelude::{Bech32Address, Provider, TxParameters},
    signers::WalletUnlocked,
    tx::{Address, AssetId, Contract, Input, TxPointer, UtxoId},
    core::types::Bits256
};

/// Gets the message to contract predicate

pub async fn create_order(
    maker: &WalletUnlocked,
    order: &LimitOrder,
    provider: &Provider,
) -> (Predicate, Input) {

    create_predicate(
        "0x7895d0059c0d0c1de8de15795191a1c1d01cd970db75fa42e15dc96e051b5570".to_string(),
        "1_000_000".to_string(),
        "0u8".to_string(),
        maker.address(),
        order.maker_amount,
        order.taker_amount, 
        order.maker_token, 
        order_taker.token,
        "123123".to_string()
    );

    let predicate_bytecode = compile_to_bytes("order-predicate.sw", true).into_bytes();
    let predicate = Predicate::new(predicate_bytecode.clone());
    let predicate_root = Address::from(*Contract::root_from_code(predicate_bytecode.clone()));
    
    // create the order (fund the predicate)
    let (_tx, _rec) = maker
        .transfer(
            predicate.address(),
            order.maker_amount,
            AssetId::from(order.maker_token.0),
            TxParameters::default(),
        )
        .await
        .unwrap();
    let predicate_coin = &provider
        .get_coins(&predicate_root.into(), AssetId::default())
        .await
        .unwrap()[0];
    let predicate_coin_input = Input::CoinPredicate {
        utxo_id: UtxoId::from(predicate_coin.utxo_id.clone()),
        owner: predicate_root,
        amount: order.maker_amount,
        asset_id: AssetId::from(order.maker_token.0),
        tx_pointer: TxPointer::default(),
        maturity: 0,
        predicate: predicate_bytecode.clone(),
        predicate_data: vec![],
    };
    (predicate, predicate_coin_input)
}


