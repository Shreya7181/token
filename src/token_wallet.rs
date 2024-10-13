#[canister] 
mod token { 
    use ic_cdk::export::candid::{CandidType, Deserialize}; 
    use ic_cdk::api::call; 
    use std::collections::HashMap; 
    
    #[derive(CandidType, Deserialize)] 
    pub struct Wallet { 
        pub balances: HashMap<String, u64>, 
    } 
    // Initialize the wallet with some default balances (for testing purposes) 
    #[update] 
    fn init() { 
        let mut initial_balances = HashMap::new();
        initial_balances.insert(ic_cdk::caller().to_string(), 1000); 
        // Start with 1000 tokens for the caller 
        let wallet = Wallet { balances: initial_balances, 
        }; 
        ic_cdk::storage::set(wallet); 
    } 
    #[update] 
    fn send_tokens(to: String, amount: u64) {
         let mut wallet: Wallet = ic_cdk::storage::get(); 
         let caller = ic_cdk::caller().to_string(); 
         // Check if the sender has enough balance 
         let sender_balance = wallet.balances.entry(caller.clone()).or_insert(0); 
         if *sender_balance < amount { 
            panic!("Insufficient balance.");
         }
          // Deduct tokens from sender 
         *sender_balance -= amount; 
          // Add tokens to the recipient 
          let recipient_balance = wallet.balances.entry(to.clone()).or_insert(0); 
          *recipient_balance += amount; // Update the storage 
          ic_cdk::storage::set(wallet); 
        } 
        #[update] 
        fn receive_tokens(from: String, amount: u64) { 
            let mut wallet: Wallet = ic_cdk::storage::get(); 
            let recipient = ic_cdk::caller().to_string(); // Add tokens to the recipient 
            let recipient_balance = wallet.balances.entry(recipient.clone()).or_insert(0); 
            *recipient_balance += amount; // Update the storage 
            ic_cdk::storage::set(wallet); 
        } 
        #[query] 
        fn get_balance(address: String) -> u64 { 
            let wallet: Wallet = ic_cdk::storage::get(); 
            *wallet.balances.get(&address).unwrap_or(&0) // Return balance or 0 if not found 
            } 
        }
