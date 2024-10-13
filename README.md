# MyTokenWallet 

Welcome to your new token wallet project built on the Internet Computer Protocol (ICP). This wallet supports basic functionalities for managing IRCRC2 tokens, including sending and receiving tokens, as well as checking balances. 

## Objective 

Create a Rust-based token wallet for the ICP blockchain. The wallet should demonstrate proficiency in Rust and blockchain principles while supporting the following features: - Send tokens to other addresses - Receive tokens and update balances - Display current token balances 

## Features - 
**Token Management**: 
- Send and receive tokens. 
- Check balances of different addresses. 
- **Security**: Basic security features implemented to protect wallet transactions. 

## Getting Started 
### Prerequisites 
- Rust (version 1.81.0 or higher) 
- DFX (Internet Computer SDK) 
- Internet connection for installation 

### Installation 
### Steps to Install
 1. **Install dependencies:** 
 Make sure you have Rust and Cargo installed. If not, install them by running: 
 ```bash 
 curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 
 
 2. Install DFX SDK: Run the following command to install the DFX SDK: 
 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)" 
 
 3. Start local ICP network: 
 Start the local Internet Computer network by running: 
 dfx start 
 
 4. Deploy the canisters:
  Navigate to your project directory and deploy the canisters: 
  dfx deploy 
  
  Usage After deployment, you can interact with the wallet through the front-end interface. Follow the on-screen instructions to send and receive tokens. 
  
  Functionality 
  • Send Tokens: Users can send tokens to other addresses. 
  • Receive Tokens: The wallet can receive tokens and update the balance accordingly. 
  • Balance Display: Current token balance can be displayed. 
  
  Testing 
  
  To run tests for the smart contracts, use the following command: 
  
  cargo test
