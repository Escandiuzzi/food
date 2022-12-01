use crate::entities::Entity;
use crate::payments::Wallet;

pub struct Customer {
    name: String,
    wallet: Wallet
}

impl Customer {
    pub fn new(name: String, wallet: Wallet) -> Customer {
        return Customer {
            name: name,
            wallet: wallet
        }
    }
}

impl Entity for Customer {
    fn get_name(&self) -> &str {
        return &self.name;
    }
    
    fn get_cash_quantity(&self) -> &f32 {
        return &self.wallet.get_amount();
    }

    fn charge(&self, amount: f32) -> bool {
        return true;
    }

    fn refund(&self, amount: f32) -> bool {
        return true;
    }

    fn notify(&self, info: &str) {
        println!("Sending this notification from Customer :) {}", info);
    }
}


#[cfg(test)]
mod test {
    use crate::entities::Customer;
    use crate::payments::Wallet;

    #[test]
    fn test_create_object() 
    {
        let name: String = "Rust".to_string();
        let amount: f32 = 1000.0;

        let customer = Customer::new("Rust".to_string(), Wallet{amount: amount});

        assert_eq!(customer.name, name);
        assert_eq!(customer.get_cash_quantity(), &amount);
    }
}
