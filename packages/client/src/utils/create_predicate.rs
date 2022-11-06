
use std::fs::File;
//use std::io::prelude::*;
use std::path::Path;

use anyhow::Result;
use forc_pkg::BuiltPackage;
use forc_pkg::PackageManifestFile;
use regex::{Captures , Regex};
use std::{fmt::Write, io::Read, path::PathBuf};
// use fuels::{tx::Address};

pub fn compile_to_bytes(
    file_name: &str,
    capture_output: bool,
) -> (Result<BuiltPackage>, String) {
    tracing::info!(" Compiling {}", file_name);

    let mut buf_stdout: Option<gag::BufferRedirect> = None;
    let mut buf_stderr: Option<gag::BufferRedirect> = None;
    if capture_output {
        // Capture both stdout and stderr to buffers, compile the test and save to a string.
        buf_stdout = Some(gag::BufferRedirect::stdout().unwrap());
        buf_stderr = Some(gag::BufferRedirect::stderr().unwrap());
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = format!(
        "{}/src/e2e_vm_tests/test_programs/{}",
        manifest_dir, file_name
    );
    let manifest = PackageManifestFile::from_dir(&PathBuf::from(path)).unwrap();
    let result = forc_pkg::build_package_with_options(
        &manifest,
        forc_pkg::BuildOpts {
            pkg: forc_pkg::PkgOpts {
                path: Some(format!(
                    "{}/src/e2e_vm_tests/test_programs/{}",
                    manifest_dir, file_name
                )),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let mut output = String::new();
    if capture_output {
        let mut buf_stdout = buf_stdout.unwrap();
        let mut buf_stderr = buf_stderr.unwrap();
        buf_stdout.read_to_string(&mut output).unwrap();
        buf_stderr.read_to_string(&mut output).unwrap();
        drop(buf_stdout);
        drop(buf_stderr);

        // Capture the result of the compilation (i.e., any errors Forc produces) and append to
        // the stdout from the compiler.
        if let Err(ref e) = result {
            write!(output, "\n{}", e).expect("error writing output");
        }

        if cfg!(windows) {
            // In windows output error and warning path files start with \\?\
            // We replace \ by / so tests can check unix paths only
            let regex = Regex::new(r"\\\\?\\(.*)").unwrap();
            output = regex
                .replace_all(output.as_str(), |caps: &Captures| {
                    caps[1].replace('\\', "/")
                })
                .to_string();
        }
    }

    (result, output)
}

pub fn create_predicate(SPENDING_SCRIPT_HASH:u64, MIN_GAS:u64, OUTPUT_COIN_INDEX:u8, MAKER_ADDRESS:u64, MAKER_AMOUNT:u64, TAKER_AMOUNT:u64, SALT: u8, MAKER_TOKEN:u64, TAKER_TOKEN:u64, MSG_SENDER: u64) {
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
    match file.write_all(template.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
