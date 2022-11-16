mod common;

use fuels::{
    prelude::*,
    tx::{AssetId, Input, TxPointer},
};

use client::utils::{build_take_order::LimitOrder, order::*};

pub async fn verify_balance_post_swap(
    maker: &WalletUnlocked,
    taker: &WalletUnlocked,
    predicate_address: &Bech32Address,
    order: LimitOrder,
    provider: &Provider,
) {
    let maker_token = AssetId::from(order.maker_token.0);
    let taker_token = AssetId::from(order.taker_token.0);
    let balance_maker = maker.get_asset_balance(&taker_token).await.unwrap();
    let balance_taker = taker.get_asset_balance(&maker_token).await.unwrap();
    let predicate_balance = provider
        .get_asset_balance(predicate_address, maker_token)
        .await
        .unwrap();
    assert!(balance_maker == order.taker_amount);
    assert!(balance_taker == order.maker_amount);
    assert!(predicate_balance == 0);
}

#[tokio::test]
async fn test_swap() {
    let (provider, maker, _, taker, ..) = common::setup().await;

    let order = LimitOrder {
        maker: maker.address().into(),
        maker_amount: 10,
        taker_amount: 10,
        maker_token: Bits256::from_token(Token::B256([0u8; 32])).unwrap(),
        taker_token: Bits256::from_token(Token::B256([1u8; 32])).unwrap(),
        salt: 22,
    };

    let (predicate, predicate_coin_input) = create_order(&maker, &order, &provider).await;
    let gas_coin = &provider
        .get_spendable_coins(taker.address(), AssetId::default(), 1)
        .await
        .unwrap()[1];
    let gas_coin_utxo_id = gas_coin.utxo_id.clone().into();
    let gas_coin_amount: u64 = gas_coin.amount.clone().into();

    let input_gas = Input::CoinSigned {
        utxo_id: gas_coin_utxo_id,
        tx_pointer: TxPointer::default(),
        owner: Address::from(taker.address()),
        amount: gas_coin_amount,
        asset_id: AssetId::default(),
        witness_index: 0,
        maturity: 0,
    };

    take_order(&taker, &order, &provider, input_gas, predicate_coin_input).await;
    verify_balance_post_swap(&maker, &taker, predicate.address(), order, &provider).await;
}
