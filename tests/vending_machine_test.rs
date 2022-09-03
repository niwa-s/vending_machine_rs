use tdd_vending_machine::{Coin, VendingMachine};

// コーラを購入できるか・コインがたりない場合は購入できないか・多めに投入しても購入してもらえるか
#[test]
fn buy_cola() {
    let mut machine = VendingMachine::new();
    machine.insert_coin(Coin::try_from(100).unwrap());

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
    let coins = [10, 100, 500]
        .iter()
        .map(|&value| Coin::try_from(value).unwrap());
    machine.insert_coins(coins);

    let beverage = machine.press_button(0).unwrap();
    assert_eq!(beverage, "Cola");
}

// コーラ以外も購入できるかテスト
#[test]
fn buy_woo_long_tea() {
    let mut machine = VendingMachine::new();
    machine.insert_coin(Coin::try_from(100).unwrap());

    let beverage = machine.press_button(1).unwrap();
    assert_eq!(beverage, "Woo long tea");
}
#[test]
fn buy_ilohas() {
    let mut machine = VendingMachine::new();
    machine.insert_coin(Coin::try_from(100).unwrap());

    let beverage = machine.press_button(2).unwrap();
    assert_eq!(beverage, "Ilohas");
}
#[test]
fn buy_red_bull() {
    let mut machine = VendingMachine::new();
    let coins = [100, 100]
        .iter()
        .map(|&value| Coin::try_from(value).unwrap());
    machine.insert_coins(coins);

    let beverage = machine.press_button(3).unwrap();
    assert_eq!(beverage, "Red Bull");
}
#[test]
fn use_invalid_button_id() {
    let mut machine = VendingMachine::new();
    machine.insert_coin(Coin::try_from(100).unwrap());

    let press_button_result = machine.press_button(4);
    assert!(press_button_result.is_err());
}

// 購入ボタンが光っているかどうかのテスト
#[test]
fn button_is_shining_with_enough_coin() {
    let mut machine = VendingMachine::new();
    machine.insert_coin(Coin::try_from(100).unwrap());

    assert!(machine.is_button_shining(0).unwrap());
}
#[test]
fn button_is_not_shining_without_enough_coin() {
    let mut machine = VendingMachine::new();
    machine.insert_coin(Coin::try_from(10).unwrap());

    assert!(!machine.is_button_shining(0).unwrap());
}

// 小銭で購入できるかテスト
#[test]
fn buy_cola_for_10_yen_coin() {
    let mut machine = VendingMachine::new();
    let coins = [10; 10].iter().map(|&value| Coin::try_from(value).unwrap());
    machine.insert_coins(coins);

    let beverage = machine.press_button(0).unwrap();
    assert_eq!(beverage, "Cola");
}
#[test]
fn buy_cola_for_50_yen_coin() {
    let mut machine = VendingMachine::new();
    let coins = [50, 50].iter().map(|&value| Coin::try_from(value).unwrap());
    machine.insert_coins(coins);

    let beverage = machine.press_button(0).unwrap();
    assert_eq!(beverage, "Cola");
}

// お釣りの金額が正しいかテスト
#[test]
fn buy_cola_for_150_yen() {
    let mut machine = VendingMachine::new();
    let coins = [50, 100]
        .iter()
        .map(|&value| Coin::try_from(value).unwrap());
    machine.insert_coins(coins);

    let beverage = machine.press_button(0).unwrap();
    assert_eq!(beverage, "Cola");
    assert_eq!(machine.return_coins().sum(), 50);
}
#[test]
fn buy_woo_long_tea_for_120_yen() {
    let mut machine = VendingMachine::new();
    let coins = [10, 10, 100]
        .iter()
        .map(|&value| Coin::try_from(value).unwrap());
    machine.insert_coins(coins);

    let beverage = machine.press_button(1).unwrap();
    assert_eq!(beverage, "Woo long tea");
    let change = machine.return_coins().sum();
    assert_eq!(change, 20);
}
#[test]
fn return_change_test() {
    let mut machine = VendingMachine::new();
    machine.insert_coin(Coin::try_from(100).unwrap());

    let change = machine.return_coins().sum();
    assert_eq!(change, 100);
}
