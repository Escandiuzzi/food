use crate::entities::{Customer, Restaurant, Rider};
use crate::payments::{Wallet};

pub struct Order {
    pub customer: Customer,
    pub restaurant: Restaurant,
    pub rider: Rider
}

impl Order {
    pub fn new(customer: Customer, restaurant: Restaurant, rider: Rider) -> Order {
        return Order{ customer: customer, restaurant: restaurant, rider: rider };
    }

    pub fn create() -> Order {

        let customer: Customer = create_customer();
        let restaurant: Restaurant = create_restaurant();
        let rider: Rider = create_rider();

        return Order::new(customer, restaurant, rider);
    }
}

fn create_customer() -> Customer {
    let name: String = "Customer".to_string();
    let amount: f32 = 1000.0;

    let wallet = Wallet::new(amount);
    return Customer::new(name, wallet);
}

fn create_restaurant() -> Restaurant {
    let name: String = "Restaurant".to_string();
    let amount: f32 = 1000.0;

    let wallet = Wallet::new(amount);
    return Restaurant::new(name, wallet);
}

fn create_rider() -> Rider {
    let name: String = "Rider".to_string();
    let amount: f32 = 1000.0;

    let wallet = Wallet::new(amount);
    return Rider::new(name, wallet);
}