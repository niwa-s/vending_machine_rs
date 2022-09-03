use std::ops::Deref;

const ALLOWED_VALUES: &[u32; 4] = &[10, 50, 100, 500];
fn validate(value: u32) -> bool {
    ALLOWED_VALUES.contains(&value)
}


#[derive(Debug, Clone, Copy, Default)]
pub struct Coin {
    value: u32,
}

impl Coin {
    pub fn value(&self) -> u32 {
        self.value
    }
}
impl TryFrom<u32> for Coin {
    type Error = anyhow::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if validate(value) {
            Ok(Self { value })
        } else {
            Err(anyhow::anyhow!("Invalid coin value: {}", value))
        }
    }
}

#[derive(Debug, Default)]
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
    pub fn extend(&mut self, coins: impl IntoIterator<Item = Coin>) {
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
            coins.extend(vec![Coin::try_from(coin_value).unwrap(); count as usize]);
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