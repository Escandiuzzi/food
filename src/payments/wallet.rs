pub struct Wallet {
    amount: f32
}

impl Wallet {
    pub fn new(amount: f32) -> Wallet {
        return Wallet{amount: amount};
    }

    pub fn get_amount(&self) -> &f32 {
        return &self.amount;
    }

    pub fn deduct(&self, amount: &f32) -> bool {
        if(&self.amount > amount) {
            &self.amount - amount;
            return true;
        } else {
            return false;
        }
    }

    pub fn add(&self, amount: &f32) {
        &self.amount + amount;
    }
}