use std::collections::HashMap;

#[derive(Debug, Clone)]
struct EthereumAccount {
    address: String,   
    balance: u64, 
}

impl EthereumAccount {
    fn new(address: String, initial_balance: u64) -> Self {
        Self {
            address,
            balance: initial_balance,
        }
    }

    fn transfer(&mut self, recipient: &mut EthereumAccount, amount: u64) -> Result<(), String> {
        if self.balance < amount {
            return Err(format!(
                "Insufficient funds: {} Wei available, {} Wei required",
                self.balance, amount
            ));
        }

        self.balance -= amount;
        recipient.balance += amount;
        Ok(())
    }

    fn balance_in_ether(&self) -> f64 {
        self.balance as f64 / 1_000_000_000_000_000_000.0
    }
}

struct EthereumNetwork {
    accounts: HashMap<String, EthereumAccount>, 
}

impl EthereumNetwork {
    fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    fn add_account(&mut self, account: EthereumAccount) {
        self.accounts.insert(account.address.clone(), account);
    }

    fn get_account(&self, address: &str) -> Option<&EthereumAccount> {
        self.accounts.get(address)
    }

    fn get_account_mut(&mut self, address: &str) -> Option<&mut EthereumAccount> {
        self.accounts.get_mut(address)
    }
}

fn main() {
    let mut account1 = EthereumAccount::new("0xAccount1".to_string(), 5_000_000_000_000_000_000); 
    let mut account2 = EthereumAccount::new("0xAccount2".to_string(), 3_000_000_000_000_000_000); 

    let mut network = EthereumNetwork::new();
    network.add_account(account1.clone());
    network.add_account(account2.clone());

    match account1.transfer(&mut account2, 2_000_000_000_000_000_000) { 
        Ok(_) => println!("Transfer successful!"),
        Err(err) => println!("Transfer failed: {}", err),
    }

    println!("Account 1 balance: {} Ether", account1.balance_in_ether());
    println!("Account 2 balance: {} Ether", account2.balance_in_ether());

    if let Some(acc) = network.get_account("0xAccount1") {
        println!("Account 1 on network: {:?}", acc);
    }
}
