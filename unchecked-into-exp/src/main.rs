use std::mem;
fn main() {
    let x: u32 = 42; 
    let y: f32 = unsafe{mem::transmute_copy(&x)}; 

    println!("Printing X {:?} ", x); 
    println!("Printing Y {:?} ", y); 

}


fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {