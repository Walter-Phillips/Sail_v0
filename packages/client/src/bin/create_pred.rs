// dynamically builds the predicate and compiles to binary
use std::fs::File;
//use std::io::prelude::*;
use std::path::Path;
use std::io::Write;

use anyhow::Result;
use forc_pkg::BuiltPackage;
use forc_pkg::PackageManifestFile;
use regex::{Captures , Regex};
use std::{fmt::Write, io::Read, path::PathBuf};
// use fuels::{tx::Address};


pub fn create_predicate(SPENDING_SCRIPT_HASH:str, MIN_GAS:str, OUTPUT_COIN_INDEX:str, MAKER_ADDRESS:str, MAKER_AMOUNT:str, TAKER_AMOUNT:str, SALT: str, MAKER_TOKEN:str, TAKER_TOKEN:str, MSG_SENDER: str) {
    let template =
        format!("predicate;
        use std::{{
            b512::B512,
            constants::ZERO_B256,
            ecr::ec_recover_address,
            inputs::input_predicate_data,
            revert::require,
        }};
        use order::*;
        
        // update this with the script for spending
        const SPENDING_SCRIPT_HASH = {};
        // const MIN_GAS = {};
        // the constants that define each predicate. I would rather pass these as arguments, but i dont know how 
        const OUTPUT_COIN_INDEX = {};
        fn main(take_coin: b256, min_take_amount: u64, maker: b256) -> bool {{
            // parameterize this thing
            let order = LimitOrder {{
                maker: Address::from({}),
                maker_amount: {},
                taker_amount: {},
                maker_token: {},
                taker_token: {},
                salt: {},
            }};
        
            // handle cancellations
            let msg_sender = {}
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
        
            // this is the one that is failing, its because maker above is set to 0, which is incorrect
            // just need to pass this thing in args (along with all other params)
            assert(output_coin_to(OUTPUT_COIN_INDEX) == order.maker.into());
            true
        }}
        
        ////////////
        // Inuput //
        ////////////
        const GTF_INPUT_COIN_AMOUNT = 0x105;
        const GTF_INPUT_COIN_ASSET_ID = 0x106;
        const GTF_SCRIPT_SCRIPT_LENGTH = 0x005;
        const GTF_SCRIPT_SCRIPT = 0x00B;
        pub fn input_coin_asset_id(index: u64) -> b256 {{
            __gtf::<b256>(index, GTF_INPUT_COIN_ASSET_ID)
        }}
        
        /// Get the amount of a coin input
        pub fn input_coin_amount(index: u64) -> u64 {{
            __gtf::<u64>(index, GTF_INPUT_COIN_AMOUNT)
        }}
        
        /// Get the hash of the script bytecode
        pub fn tx_script_bytecode_hash() -> b256 {{
            let mut result_buffer = ZERO_B256;
            asm(hash: result_buffer, ptr: __gtf::<u64>(0, GTF_SCRIPT_SCRIPT), len: __gtf::<u64>(0, GTF_SCRIPT_SCRIPT_LENGTH)) {{
                s256 hash ptr len;
                hash: b256
            }}
        }}
        const GTF_SCRIPT_GAS_PRICE = 0x002;
        const GTF_SCRIPT_GAS_LIMIT = 0x003;
        /// Get the transaction gas price
        pub fn tx_gas_price() -> u64 {{
            __gtf::<u64>(0, GTF_SCRIPT_GAS_PRICE)
        }}
        
        /// Get the transaction gas price
        pub fn tx_gas_limit() -> u64 {{
            __gtf::<u64>(0, GTF_SCRIPT_GAS_LIMIT)
        }}
        
        ////////////
        // OUTPUT //
        ////////////
        /// Get the transaction outputs count
        const GTF_SCRIPT_OUTPUTS_COUNT = 0x008;
        const GTF_OUTPUT_TYPE = 0x201;
        const OUTPUT_TYPE_COIN = 0u8; // again... not sure aboue this type here. 
        const GTF_OUTPUT_COIN_TO: u64 = 0x202;
        const GTF_OUTPUT_COIN_AMOUNT: u64 = 0x203;
        const GTF_OUTPUT_COIN_ASSET_ID: u64 = 0x204;
        pub fn output_count() -> u64 {{
            __gtf::<u64>(0, GTF_SCRIPT_OUTPUTS_COUNT)
        }}
        fn verify_output_coin(index: u64) -> bool {{
            __gtf::<u64>(index, GTF_OUTPUT_TYPE) == OUTPUT_TYPE_COIN
        }}
        
        fn output_coin_asset_id(index: u64) -> b256 {{
            __gtf::<b256>(index, GTF_INPUT_COIN_ASSET_ID)
        }}
        fn output_coin_amount(index: u64) -> u64 {{
            __gtf::<u64>(index, GTF_OUTPUT_COIN_AMOUNT)
        }}
        fn output_coin_to(index: u64) -> b256 {{
            __gtf::<b256>(index, GTF_OUTPUT_COIN_TO)
        }}
        
    ", SPENDING_SCRIPT_HASH, MIN_GAS, OUTPUT_COIN_INDEX, MAKER_ADDRESS, MAKER_AMOUNT, TAKER_AMOUNT, SALT, MAKER_TOKEN, TAKER_TOKEN, MSG_SENDER);
    
        let path = Path::new("order-predicate.sw");
        let display = path.display();
    
        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
    
        // Write the template params to `file`, returns `io::Result<()>`
        match file.write(template.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
}