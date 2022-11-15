predicate;

    use std::{
        b512::B512,
        constants::ZERO_B256,
        ecr::ec_recover_address,
        inputs::input_predicate_data,
        revert::require,
    };
    use order::*;
    
    // update this with the script for spending
    const SPENDING_SCRIPT_HASH = 0x7895d0059c0d0c1de8de15795191a1c1d01cd970db75fa42e15dc96e051b5570;
    const MIN_GAS = 1_000_000;
    const OUTPUT_COIN_INDEX = 0u8;
    fn main() -> bool {
        // parameterize this 
        let order = LimitOrder {
            maker: Address::from(fuel1uy84y6ceykfhjwm6z4v68y2yt746s2sav60ravku69lecysmyjcss0yrdx),
            maker_amount: 10,
            taker_amount: 10,
            maker_token: Bits256([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            taker_token: Bits256([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            salt: 123123,
        };

        let sender: Result<Identity, AuthError> = msg_sender();
        if(sender.unwrap() == order.maker) {
            true
        }
    
        ////////////
        // INPUTS //
        ////////////
        assert(tx_script_bytecode_hash() == SPENDING_SCRIPT_HASH);
        assert(input_coin_asset_id(0) == order.taker_token);
        assert(input_coin_amount(0) >= order.taker_amount);
    
        // todo: The gas coin stuff, note: that if this is the same coin as the take
        // coin then we will need to verify slightly differently
        // let gas_coin = input_coin_asset_id(1);
        // let gas_coin_amount = input_coin_amount(1);
        // assert(gas_coin_amount >= tx_gas_price() * MIN_GAS);
        // assert(tx_gas_limit() >= MIN_GAS);
        /////////////
        // OUTPUTS //
        /////////////
        assert(output_count() == 2);
        assert(verify_output_coin(OUTPUT_COIN_INDEX));
        assert(output_coin_asset_id(OUTPUT_COIN_INDEX) == order.taker_token);
        assert(output_coin_amount(OUTPUT_COIN_INDEX) >= order.taker_amount);
    
        assert(output_coin_to(OUTPUT_COIN_INDEX) == order.maker.into());
        true
    }
    
    ////////////
    // Input //
    ////////////
    const GTF_INPUT_COIN_AMOUNT = 0x105;
    const GTF_INPUT_COIN_ASSET_ID = 0x106;
    const GTF_SCRIPT_SCRIPT_LENGTH = 0x005;
    const GTF_SCRIPT_SCRIPT = 0x00B;
    pub fn input_coin_asset_id(index: u64) -> b256 {
        __gtf::<b256>(index, GTF_INPUT_COIN_ASSET_ID)
    }
    
    /// Get the amount of a coin input
    pub fn input_coin_amount(index: u64) -> u64 {
        __gtf::<u64>(index, GTF_INPUT_COIN_AMOUNT)
    }
    
    /// Get the hash of the script bytecode
    pub fn tx_script_bytecode_hash() -> b256 {
        let mut result_buffer = ZERO_B256;
        asm(hash: result_buffer, ptr: __gtf::<u64>(0, GTF_SCRIPT_SCRIPT), len: __gtf::<u64>(0, GTF_SCRIPT_SCRIPT_LENGTH)) {
            s256 hash ptr len;
            hash: b256
        }
    }
    const GTF_SCRIPT_GAS_PRICE = 0x002;
    const GTF_SCRIPT_GAS_LIMIT = 0x003;
    /// Get the transaction gas price
    pub fn tx_gas_price() -> u64 {
        __gtf::<u64>(0, GTF_SCRIPT_GAS_PRICE)
    }
    
    /// Get the transaction gas price
    pub fn tx_gas_limit() -> u64 {
        __gtf::<u64>(0, GTF_SCRIPT_GAS_LIMIT)
    }
    
    ////////////
    // OUTPUT //
    ////////////
    /// Get the transaction outputs count
    const GTF_SCRIPT_OUTPUTS_COUNT = 0x008;
    const GTF_OUTPUT_TYPE = 0x201;
    const OUTPUT_TYPE_COIN = 0u8; 
    const GTF_OUTPUT_COIN_TO: u64 = 0x202;
    const GTF_OUTPUT_COIN_AMOUNT: u64 = 0x203;
    const GTF_OUTPUT_COIN_ASSET_ID: u64 = 0x204;
    pub fn output_count() -> u64 {
        __gtf::<u64>(0, GTF_SCRIPT_OUTPUTS_COUNT)
    }
    fn verify_output_coin(index: u64) -> bool {
        __gtf::<u64>(index, GTF_OUTPUT_TYPE) == OUTPUT_TYPE_COIN
    }
    
    fn output_coin_asset_id(index: u64) -> b256 {
        __gtf::<b256>(index, GTF_INPUT_COIN_ASSET_ID)
    }
    fn output_coin_amount(index: u64) -> u64 {
        __gtf::<u64>(index, GTF_OUTPUT_COIN_AMOUNT)
    }
    fn output_coin_to(index: u64) -> b256 {
        __gtf::<b256>(index, GTF_OUTPUT_COIN_TO)
    }