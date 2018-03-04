// Inspired by https://www.ethereum.org/token

use std::collections::HashMap;

use libcontract_common::{Address, Contract, ContractError};

use token_api::TokenState; // TODO: what does this do?

pub struct DexContract {
    // Map Account hash to mapping {token_currency: balance}
    // TODO: Evaluate efficency of structures
    accounts: HashMap<u64,HashMap<String,u64>>,
    // Map user: {Token_desired: amount_desired}
    purchase_orders: HashMap<u64,HashMap<String,u64>>,
    // map user: {Token_selling: Amount_selling}
    sell_orders: HashMap<u64,HashMap<String,u64>>,
}

impl DexContract {
    pub fn new(
        accounts: HashMap<u64,HashMap<String,u64>>,
        purchase_orders: HashMap<u64,HashMap<String,u64>>,
        sell_orders: HashMap<u64,HashMap<String,u64>>,
    ) -> TokenContract {
        let mut accounts_new = HashMap::new();
        let mut purchase_orders_new =  HashMap::new();
        let mut sell_orders_new =  HashMap::new();
        TokenContract {
            accounts: accounts_new,
            purchase_orders: purchase_orders_new,
            sell_orders: sell_orders_new
        }
    }
    // Private methods
    fn get_user_balance(&self,user_account_address:&Address,token_type: String) -> Result<u64,ContractError> {
        let account = self.accounts.get(user_account_address);
        if (account == None) {
            return Err(ContractError::new("Nonexistent User"));
        }
        value = account.get(token_type);
        if (value == None) {
            return Err(ContractError::new("No such token held by user"));
        }
        return value;
    }


    // TODO: Verify source of deposit and execute transaction logic
    fn deposit(&self, user_account_address:&Address, token_type:String,amount: u64) -> Result<(),ContractError> {
        let account = self.accounts.get(user_account_address);
        if (account == None) {
            return Err(ContractError::new("No such user"));
        }
        let value = account.get(token_type);
        let insertion = value.entry(token_type).or_insert(0);
        *insertion += amount;
        return ();
    }

    // TODO: Verify source of withdrawal and execute transaction logic.
    fn withdraw(&self, user_account_address: &Address, token_type: String, amount: u64) -> Result<(),ContractError> {
        let account = self.accounts.get(user_account_address);
        if (account == None) {
            return Err(ContractError::new("No such user"));
        }
        let value = account.get(token_type);
        if (value == None) {
            return Err(ContractError::new("Currency not held by user"));
        }
        if (value < amount) {
            return Err(ContractError::new("Insufficient Funds"));
        }
        *account.get_mut(token_type).unwrap() -= amount;
        return ();

    }
    // Users place an order to buy currency A using currency B
    fn place_order(&self,user_account_address: &Address,
                   from_token_type: String, to_token_type: String,
                   from_amount: u64, to_amount: u64) -> Result<(),ContractError> {
        account = self.accounts.get(user_account_address);
        purchase_needed = true;
        sale_needed = true;

        if (account == None) {
            return Err(ContractError::new("NO SUCH USER"))
        }
        if (account.get(from_token_type) < from_amount) {
            return Err(ContractError::new("INSUFFICIENT FUNDS"));
        }
        // If corresponding sale record in entry: update, else add to sell orders
        for (user,sale) in &(self.sell_orders) {
            // try to purchase a token from the sale list
            if (sale.get(to_token_type) != None && sale.get(to_token_type) >= to_amount) {
                *sale.get_mut(to_token_type).unwrap() -= to_amount;

                destination_user = self.accounts.get(user);
                *destination_user.get_mut(to_token_type).unwrap() -= amount; // TODO: Also credit the user in the currency requested
                purchase_needed = false;
                break;
            }
        }
        // check if we can automatically sell our token
        for (user,purchase) in &(self.buy_orders) {
            if (purchase.get(from_token_type) != None && purchase.get(from_amount) > from_amount) {
                *purchase.get_mut(from_token_type).unwrap() -= from_amount;
                *account.entry(to_token_type).or_insert(to_amount);
                // Credit the user who purchased our coins, TODO: This user also needs to be debited in the currency of his purchase
                destination_user = self.accounts.get(user);
                *destination_user.get_mut(from_token_type).unwrap() += from_amount;

                sale_needed = false;
                break;
            }
        }

        if purchase_needed {
            // Create buy and sell entries TODO: These need to be linked somehow
            sale_user = self.sell_orders.get(user_account_address);
            if (sale_user == None || sale_user.get(from_token_type) == None) {
                let mut user_order = HashMap::new();
                user_order.insert(from_token_type, from_amount);
                self.sell_orders.insert(user_account_address, user_order);
            } else {
                *sale_user.get_mut(from_token_type).unwrap() += from_amount;
            }
        }
        if sale_needed {
            buy_user = self.buy_orders.get(user_account_address);
            if (buy_user == None || buy_user.get(to_token_type) == None) { // If user doesn't already have a longstanding order for this currency
                let mut user_order = HashMap::new();
                user_order.insert(to_token_type,to_amount);
                self.buy_orders.insert(user_account_address,user_order);
            } else {
                *buy_user.get_mut(to_token_type).unwrap() += to_amount;
            }
        }




    }



    // Preserved from Token Contract as example
//    fn get_from_balance(&self, addr: &Address, value: u64) -> Result<u64, ContractError> {
//        match self.balance_of.get(addr.as_str()) {
//            None => Err(ContractError::new("Nonexistent `from` account")),
//            Some(b) if *b < value => Err(ContractError::new("Insufficient `from` balance")),
//            Some(b) => Ok(*b),
//        }
//    }





    // PUBLIC METHODS
    // - callable over RPC
    pub fn get_name(&self) -> Result<String, ContractError> {
        Ok(self.name.clone())
    }

    pub fn get_symbol(&self) -> Result<String, ContractError> {
        Ok(self.symbol.clone())
    }

    pub fn get_balance(&self, msg_sender: &Address) -> Result<u64, ContractError> {
        self.get_to_balance(msg_sender)
    }

    pub fn transfer(
        &mut self,
        msg_sender: &Address,
        to: &Address,
        value: u64,
    ) -> Result<(), ContractError> {
        self.do_transfer(msg_sender, to, value)
    }

    pub fn burn(&mut self, msg_sender: &Address, value: u64) -> Result<(), ContractError> {
        let from_balance = self.get_from_balance(msg_sender, value)?;
        self.balance_of
            .insert(msg_sender.to_string(), from_balance - value);
        self.total_supply -= value;
        // Emit Burn(msg_sender, value) event;
        Ok(())
    }
}

impl Contract<TokenState> for TokenContract {
    /// Get serializable contract state.
    fn get_state(&self) -> TokenState {
        let mut state = TokenState::new();
        state.set_name(self.name.clone());
        state.set_symbol(self.symbol.clone());
        state.set_total_supply(self.total_supply);
        state.set_balance_of(self.balance_of.clone());

        state
    }

    /// Create contract instance from serialized state.
    fn from_state(state: &TokenState) -> TokenContract {
        TokenContract {
            name: state.get_name().to_string(),
            symbol: state.get_symbol().to_string(),
            total_supply: state.get_total_supply(),
            balance_of: state.get_balance_of().clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_contract() {
        let name = "Ekiden Token";
        let symbol = "EKI";
        let a1 = Address::from(String::from("testaddr"));
        let c = TokenContract::new(&a1, 8, String::from(name), String::from(symbol));
        assert_eq!(name, c.get_name().unwrap(), "name should be set");
        assert_eq!(symbol, c.get_symbol().unwrap(), "symbol should be set");
        assert!(0 < c.total_supply, "total_supply should be set");
    }

    #[test]
    fn get_initial_balance() {
        let a1 = Address::from(String::from("testaddr"));
        let c = TokenContract::new(&a1, 8, String::from("Ekiden Tokiden"), String::from("EKI"));
        let b = c.get_balance(&a1).expect("testaddr should have tokens");
        assert_eq!(c.total_supply, b, "creator should get all the tokens");
    }

    // @todo - add more tests

}
