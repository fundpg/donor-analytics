use std::env;
use std::io;
use std::str::FromStr;
use web3::types::{Address, H160, U256, BlockNumber};
use web3::contract::Contract;

#[macro_use] extern crate rocket; 
async fn get_donations(address: &String) -> web3::Result<String> {
    // wss://mainnet.infura.io/ws/v3/c03d4301ed164a778faf942a5faa647f   
    let ws = web3::transports::WebSocket::new("wss://mainnet.infura.io/ws/v3/c03d4301ed164a778faf942a5faa647f").await?;
    let web3 = web3::Web3::new(ws);
    // let eth = web3.eth();

    // Listen for all events where: 
    let contract_address: H160 = Address::from_str("0x0F8e03A2175184228A429A118653D068F3a0Bb35").unwrap();
    let abi = include_bytes!("../abi.json");
    let contract_object = match Contract::from_json(web3.eth(), contract_address, abi) {
        Ok(contract) => contract,
        Err(e) => panic!("Error loading contract: {:?}", e),
    };

    let topic0:String= "0x0064caa73f1d59b69adbeb65654b0f0953597994e4241ee2460b560b8d65aaa2".to_string();
    let topic2:String = "0xf0572ad90c411f8f5d5dcb0938f0021e1b5d28d7cf427ddc34cc3a53cc740336".to_string();
    let event_listener = contract_object.events("Vote", topic0, "".to_string(), topic2).await?;
    // This returns a web3::contract::Error
    // I need to convert it to web3::Error

    let address: H160 = Address::from_str(&address).unwrap();
    let balance: U256 = web3.eth().balance(address,Some(BlockNumber::Latest)).await?;
    let balance: String = balance.to_string();
    Ok(balance)
}
    
    
    // (include_bytes!("../abi.json"))?;


    


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/donations/<address>")]
async fn donations(address: String) -> String {
    let donations = get_donations(&address).await.unwrap();
    let address_with_donations = format!("{} has {} donations", address, donations);

    format!("{}", address_with_donations)
}

#[launch]
fn rocket() -> _{
    rocket::build()
    .mount("/", routes![index])
    .mount("/", routes![donations])
}

