use crate::entities::Entity;
use crate::payments::Wallet;

pub struct Rider {
    name: String,
    wallet: Wallet
}

impl Rider {
    pub fn new(name: String, wallet: Wallet) -> Rider {
        return Rider {
            name: name,
            wallet: wallet
        }
    }
}

impl Entity for Rider {
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
        println!("Sending this notification from Rider :) {}", info);
    }
}