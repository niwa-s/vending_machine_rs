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

pub use coin::{Coin, Coins};

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

    fn press_button(&mut self, button_id: usize) -> Result<String> {
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

mod coin {
    use std::ops::Deref;

    const ALLOWED_VALUES: &[u32; 4] = &[10, 50, 100, 500];
    #[derive(Debug, Clone, Copy)]
    pub struct Coin {
        value: u32,
    }
    fn validate(value: u32) -> bool {
        ALLOWED_VALUES.contains(&value)
    }
    impl Coin {
        pub fn new(value: u32) -> Self {
            // TODO: error handling
            if validate(value) {}
            Self { value }
        }
        pub fn value(&self) -> u32 {
            self.value
        }
    }
    #[derive(Default)]
    pub struct Coins(Vec<Coin>);
    impl Coins {
        pub fn new() -> Self {
            Self(Vec::new())
        }
        pub fn sum(&self) -> u32 {
            self.0.iter().map(|coin| coin.value()).sum()
        }
        pub fn push(&mut self, coin: Coin) {
            self.0.push(coin);
        }
        pub fn iter(&self) -> std::slice::Iter<Coin> {
            self.0.iter()
        }
        pub fn extend<I>(&mut self, coins: I)
        where
            I: IntoIterator<Item = Coin>,
        {
            self.0.extend(coins);
        }
    }
    impl Deref for Coins {
        type Target = Vec<Coin>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl From<u32> for Coins {
        fn from(value: u32) -> Self {
            let mut coins = vec![];
            let mut value = value;
            for &coin_value in ALLOWED_VALUES.iter().rev() {
                let count = value / coin_value;
                value -= count * coin_value;
                coins.extend(vec![Coin::new(coin_value); count as usize]);
            }
            assert!(value == 0);
            Coins(coins)
        }
    }
    impl IntoIterator for Coins {
        type Item = Coin;
        type IntoIter = std::vec::IntoIter<Coin>;
        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }
    impl From<Vec<Coin>> for Coins {
        fn from(coins: Vec<Coin>) -> Self {
            Self(coins)
        }
    }
    impl From<Coins> for Vec<Coin> {
        fn from(coins: Coins) -> Self {
            coins.0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // コーラを購入できるか・コインがたりない場合は購入できないか・多めに投入しても購入してもらえるか
    #[test]
    fn buy_cola() {
        let mut machine = VendingMachine::new();
        machine.insert_coin(Coin::new(100));

        let beverage = machine.press_button(0).unwrap();
        assert_eq!(beverage, "Cola");
    }
    #[test]
    fn buy_cola_without_enough_coin() {
        let mut machine = VendingMachine::new();

        let press_button_result = machine.press_button(0);
        assert!(press_button_result.is_err());
    }
    #[test]
    fn buy_cola_with_over_coin() {
        let mut machine = VendingMachine::new();
        let coins: Coins = [10, 100, 500]
            .iter()
            .map(|&value| Coin::new(value))
            .collect::<Vec<_>>()
            .into();
        machine.insert_coins(coins);

        let beverage = machine.press_button(0).unwrap();
        assert_eq!(beverage, "Cola");
    }

    // コーラ以外も購入できるかテスト
    #[test]
    fn buy_woo_long_tea() {
        let mut machine = VendingMachine::new();
        machine.insert_coin(Coin::new(100));

        let beverage = machine.press_button(1).unwrap();
        assert_eq!(beverage, "Woo long tea");
    }
    #[test]
    fn buy_ilohas() {
        let mut machine = VendingMachine::new();
        machine.insert_coin(Coin::new(100));

        let beverage = machine.press_button(2).unwrap();
        assert_eq!(beverage, "Ilohas");
    }
    #[test]
    fn buy_red_bull() {
        let mut machine = VendingMachine::new();
        let coins = [100, 100].iter().map(|&value| Coin::new(value));
        machine.insert_coins(coins);

        let beverage = machine.press_button(3).unwrap();
        assert_eq!(beverage, "Red Bull");
    }
    #[test]
    fn use_invalid_button_id() {
        let mut machine = VendingMachine::new();
        machine.insert_coin(Coin::new(100));

        let press_button_result = machine.press_button(4);
        assert!(press_button_result.is_err());
    }

    // 購入ボタンが光っているかどうかのテスト
    #[test]
    fn button_is_shining_with_enough_coin() {
        let mut machine = VendingMachine::new();
        machine.insert_coin(Coin::new(100));

        assert!(machine.is_button_shining(0).unwrap());
    }
    #[test]
    fn button_is_not_shining_without_enough_coin() {
        let mut machine = VendingMachine::new();
        machine.insert_coin(Coin::new(10));

        assert!(!machine.is_button_shining(0).unwrap());
    }

    // 小銭で購入できるかテスト
    #[test]
    fn buy_cola_for_10_yen_coin() {
        let mut machine = VendingMachine::new();
        let coins = [10; 10].iter().map(|&value| Coin::new(value));
        machine.insert_coins(coins);

        let beverage = machine.press_button(0).unwrap();
        assert_eq!(beverage, "Cola");
    }
    #[test]
    fn buy_cola_for_50_yen_coin() {
        let mut machine = VendingMachine::new();
        let coins = [50, 50].iter().map(|&value| Coin::new(value));
        machine.insert_coins(coins);

        let beverage = machine.press_button(0).unwrap();
        assert_eq!(beverage, "Cola");
    }

    // お釣りの金額が正しいかテスト
    #[test]
    fn buy_cola_for_150_yen() {
        let mut machine = VendingMachine::new();
        let coins = [50, 100].iter().map(|&value| Coin::new(value));
        machine.insert_coins(coins);

        let beverage = machine.press_button(0).unwrap();
        assert_eq!(beverage, "Cola");
        assert_eq!(machine.return_coins().sum(), 50);
    }
    #[test]
    fn buy_woo_long_tea_for_120_yen() {
        let mut machine = VendingMachine::new();
        let coins = [10, 10, 100].iter().map(|&value| Coin::new(value));
        machine.insert_coins(coins);

        let beverage = machine.press_button(1).unwrap();
        assert_eq!(beverage, "Woo long tea");
        let change = machine.return_coins().sum();
        assert_eq!(change, 20);
    }
    #[test]
    fn return_change_test() {
        let mut machine = VendingMachine::new();
        machine.insert_coin(Coin::new(100));

        let change = machine.return_coins().sum();
        assert_eq!(change, 100);
    }
}
