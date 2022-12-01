mod entities;
mod payments;
mod order;

use crate::entities::{Entity, Customer, Restaurant, Rider};
use crate::payments::{Wallet};
use crate::order::Order;

fn main() {
    let mut order = Order::create();

    println!("Customer name from order is {}", order.customer.get_name());
}