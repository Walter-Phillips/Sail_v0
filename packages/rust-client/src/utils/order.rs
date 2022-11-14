//create the make order and the take order

use crate::utils::build_take_order::LimitOrder;
use crate::utils::create_predicate::*;

use fuels::{
    contract::predicate::Predicate,
    prelude::{Provider, TxParameters},
    signers::WalletUnlocked,
    tx::{AssetId, Address},
};

/// Gets the message to contract predicate

pub async fn create_order(
    maker: &WalletUnlocked,
    order: &LimitOrder,
) -> (Predicate, Address, Vec<u8>) {
    let (predicate, predicate_bytecode, predicate_root) = create_predicate(
        "0x7895d0059c0d0c1de8de15795191a1c1d01cd970db75fa42e15dc96e051b5570".to_string(),
        "1_000_000".to_string(),
        "0u8".to_string(),
        maker.address(),
        order.maker_amount,
        order.taker_amount, 
        order.maker_token, 
        order.taker_token,
        "123123".to_string()
    );
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
    (predicate, predicate_root, predicate_bytecode)
}
