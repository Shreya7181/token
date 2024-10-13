#[cfg(test)] 
mod tests { 
    use super::*; 
    use std::collections::HashMap; 
    #[test] 
    fn test_send_tokens() {
         let mut wallet = Wallet::new(); 
         // Call the send_tokens function 
         wallet.send_tokens("Bob".to_string(), 50); 
         // Assertions 
         assert_eq!(wallet.balances.get("Bob").unwrap(), &50);
         assert_eq!(wallet.balances.get("Alice").unwrap(), &50); 
        } 
        
        #[test] 
        fn test_receive_tokens() {
             let mut wallet = Wallet::new(); 
             wallet.balances.insert("Bob".to_string(), 50); 
             // Initial balance for Bob 
             // Call the receive_tokens function 
             wallet.receive_tokens("Bob".to_string(), 20); 
             // Assertions 
             assert_eq!(wallet.balances.get("Alice").unwrap(), &120); // Alice's new balance 
             assert_eq!(wallet.balances.get("Bob").unwrap(), &30); // Bob's new balance 
             }
            #[test] fn test_get_balance() { 
                let wallet = Wallet::new();
                 // Get balance for Alice 
                 let alice_balance = wallet.get_balance("Alice".to_string()); // Assertions
                 assert_eq!(alice_balance, 100); 
                 // Get balance for a non-existing account 
                 let non_existing_balance = wallet.get_balance("Charlie".to_string()); 
                 assert_eq!(non_existing_balance, 0); 
                } 
            }
