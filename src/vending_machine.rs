use crate::coin::{Coin, Coins};
use anyhow::Result;

#[derive(Debug, Clone, Copy)]
struct Beverage {
    name: &'static str,
    price: u32,
}
const BEVERAGE_LIST: &[Beverage; 4] = &[
    Beverage {
        name: "Cola",
        price: 100,
    },
    Beverage {
        name: "Woo long tea",
        price: 100,
    },
    Beverage {
        name: "Ilohas",
        price: 100,
    },
    Beverage {
        name: "Red Bull",
        price: 200,
    },
];

#[derive(Debug, Default)]
pub struct VendingMachine {
    coins: Coins,
}
impl VendingMachine {
    pub fn new() -> Self {
        Self {
            coins: Coins::new(),
        }
    }

    pub fn insert_coin(&mut self, coin: Coin) {
        self.coins.push(coin);
    }

    pub fn insert_coins(&mut self, coins: impl IntoIterator<Item = Coin>) {
        self.coins.extend(coins);
    }

    pub fn press_button(&mut self, button_id: usize) -> Result<String> {
        let beverage = *self.get_beverage_by_button_id(button_id)?;
        self.pay_with_coins(beverage.price)?;
        Ok(beverage.name.to_string())
    }

    pub fn is_button_shining(&self, button_id: usize) -> Result<bool> {
        let beverage = *self.get_beverage_by_button_id(button_id)?;
        let button_available = self.coins_sum() >= beverage.price;
        Ok(button_available)
    }

    fn coins_sum(&self) -> u32 {
        self.coins.sum()
    }

    fn get_beverage_by_button_id(&self, id: usize) -> Result<&Beverage> {
        BEVERAGE_LIST
            .get(id)
            .ok_or_else(|| anyhow::anyhow!("No beverage found for button_id: {}", id))
    }

    fn pay_with_coins(&mut self, price: u32) -> Result<()> {
        if self.coins_sum() < price {
            return Err(anyhow::anyhow!(
                "Not enough payment. beverage price: {}, payment: {}",
                price,
                self.coins_sum(),
            ));
        }

        let change = self.coins_sum() - price;
        self.coins = Coins::from(change);
        Ok(())
    }

    pub fn return_coins(&mut self) -> Coins {
        std::mem::take(&mut self.coins)
    }
}
