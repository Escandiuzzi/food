pub trait Entity {
    fn get_name(&self) -> &str;

    fn get_cash_quantity(&self) -> &f32;

    fn charge(&self, amount: f32) -> bool;

    fn refund(&self, amount: f32) -> bool;

    fn confirm(&self) -> bool {
        return true;
    }
    
    fn notify(&self, info: &str) {
        println!("Sending this notification from base entity :) {}", info);
    }
}