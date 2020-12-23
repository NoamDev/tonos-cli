/*
* Copyright 2018-2020 TON DEV SOLUTIONS LTD.
*
* Licensed under the SOFTWARE EVALUATION License (the "License"); you may not use
* this file except in compliance with the License.
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific TON DEV software governing permissions and
* limitations under the License.
*/
use crate::helpers::create_client_verbose;
use crate::config::Config;
use serde_json::json;
use ton_client_rs::TonAddress;

use std::fs::File;
use std::io::Write;

const ACCOUNT_FIELDS: &str = r#"
    acc_type_name
    balance
    last_paid
    last_trans_lt
    data
    boc
"#;

pub fn get_account(conf: Config, addr: &str, download: bool) -> Result<(), String> {
    let ton = create_client_verbose(&conf)?;

    TonAddress::from_str(addr)
        .map_err(|e| format!("failed to parse address: {}", e.to_string()))?;
    
    println!("Processing...");
    let query_result = ton.queries.accounts.query(
        json!({
            "id": { "eq": addr }
        }).into(),
        ACCOUNT_FIELDS,
        None,
        None,
    ).map_err(|e| format!("failed to query account info: {}", e.to_string()))?;
    println!("Succeeded.");

    if query_result.len() == 1 {
        let acc = &query_result[0];
        let acc_type = acc["acc_type_name"].as_str().unwrap();
        if acc_type != "NonExist" {
            println!("acc_type:      {}", acc_type);
            let balance_str = &acc["balance"].as_str().unwrap()[2..];
            println!("balance:       {}", u64::from_str_radix(balance_str, 16).unwrap());
            println!("last_paid:     {}", acc["last_paid"].as_u64().unwrap());
            let last_trans_lt_str = acc["last_trans_lt"].as_str().unwrap();
            println!("last_trans_lt: {}", last_trans_lt_str);
            let data_str = acc["data"].as_str();
            if data_str.is_some() {
                let data_vec = base64::decode(data_str.unwrap()).unwrap();
                println!("data(boc): {}", hex::encode(&data_vec));
            } else {
                println!("data(boc): null");
            }
            if download {
                let boc_str = acc["boc"].as_str();
                let boc_vec = base64::decode(boc_str.unwrap()).unwrap();
                let mut filename = addr.to_owned();
                filename += "-";
                filename += last_trans_lt_str;
                filename += ".tvc";
                println!("Writing BoC to file {}", filename);
                let mut file = File::create(filename).unwrap();
                file.write_all(boc_vec.as_ref()).unwrap();
            }
        } else {
            println!("Account does not exist.");    
        }
    } else {
        println!("Account not found.");
    }
    Ok(())
}
