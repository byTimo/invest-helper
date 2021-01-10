use rust_decimal::prelude::*;
use std::collections::HashMap;
use std::ops::Mul;

#[derive(Hash, Debug, Eq, PartialEq, Clone)]
enum Asset {
    Stock { ticker: String },
    Cash(Decimal),
}

type Portfolio = HashMap<Asset, u64>;
type Market = HashMap<Asset, Decimal>;
type Capital = HashMap<Asset, Decimal>;
type Strategy = HashMap<Asset, f32>;
type Transaction = (Asset, i64);

fn balance(portfolio: &Portfolio, market: &Market, strategy: &Strategy) -> Vec<Transaction> {
    let mut transactions = Vec::new();

    let mut capital = capitalize(portfolio, market);
    let total = capital
        .iter()
        .fold(Decimal::zero(), |acc, (_, price)| acc + price);
    let mut target_capital = get_target_capital(strategy, total);

    for (asset, sum) in &capital {
        if let Some(price) = market.get(asset) {
            let target_sum = target_capital.remove(asset).unwrap_or(Decimal::zero());
            match ((target_sum - sum) / price).to_i64() {
                Some(count) if count != 0 => {
                    transactions.push((asset.clone(), count))
                }
                _ => {}
            };
        }
    }

    for (asset, target_sum) in &target_capital {
        if let Some(price) = market.get(asset) {
            let sum = capital.remove(asset).unwrap_or(Decimal::zero());
            match ((target_sum - sum) / price).to_i64() {
                Some(count) if count != 0 => {
                    transactions.push((asset.clone(), count))
                }
                _ => {}
            };
        }
    };

    transactions
}

#[cfg(test)]
mod balance_tests {
    use super::*;

    #[test]
    fn empty_portfolio_empty_strategy_empty_transactions() {
        let portfolio = HashMap::new();
        let market = [
            (Asset::Stock { ticker: "ticker2".to_string() }, 10.into())
        ].iter().cloned().collect();
        let strategy = HashMap::new();

        let expected: Vec<Transaction> = Vec::new();

        let actual = balance(&portfolio, &market, &strategy);

        assert_eq!(actual, expected);
    }

    #[test]
    fn empty_market_empty_transactions() {
        let portfolio = [
            (Asset::Stock { ticker: "ticker1".to_string() }, 5)
        ].iter().cloned().collect();
        let market = HashMap::new();
        let strategy = [
            (Asset::Stock { ticker: "ticker1".to_string() }, 1.0),
        ].iter().cloned().collect();

        let expected: Vec<Transaction> = Vec::new();

        let actual = balance(&portfolio, &market, &strategy);

        assert_eq!(actual, expected);
    }

    #[test]
    fn only_cash_in_portfolio_cash_and_stock_in_strategy() {
        let portfolio = [
            (Asset::Cash(Decimal::new(6000, 0)), 1)
        ].iter().cloned().collect();
        let market = [
            (Asset::Stock { ticker: "ticker1".to_string() }, Decimal::new(7825, 2))
        ].iter().cloned().collect();
        let strategy = [
            (Asset::Stock { ticker: "ticker1".to_string() }, 0.98),
            (Asset::Cash(Decimal::zero()), 0.02),
        ].iter().cloned().collect();

        let expected: Vec<Transaction> = vec![
            (Asset::Stock { ticker: "ticker1".to_string() }, 75)
        ];

        let actual = balance(&portfolio, &market, &strategy);

        assert_eq!(actual, expected);
    }

    #[test]
    fn portfolio_have_some_part_of_strategy() {
        let portfolio = [
            (Asset::Stock { ticker: "ticker1".to_string() }, 15),
            (Asset::Cash(Decimal::new(3000, 0)), 1)
        ].iter().cloned().collect();
        let market = [
            (Asset::Stock { ticker: "ticker1".to_string() }, Decimal::new(7825, 2))
        ].iter().cloned().collect();
        let strategy = [
            (Asset::Stock { ticker: "ticker1".to_string() }, 1.0),
        ].iter().cloned().collect();

        let expected: Vec<Transaction> = vec![
            (Asset::Stock { ticker: "ticker1".to_string() }, 38)
        ];

        let actual = balance(&portfolio, &market, &strategy);

        assert_eq!(actual, expected);
    }

    #[test]
    fn portfolio_has_not_enough_cash_for_buy_transaction() {
        let portfolio = [
            (Asset::Stock { ticker: "ticker1".to_string() }, 15),
            (Asset::Cash(Decimal::new(78, 0)), 1)
        ].iter().cloned().collect();
        let market = [
            (Asset::Stock { ticker: "ticker1".to_string() }, Decimal::new(7825, 2))
        ].iter().cloned().collect();
        let strategy = [
            (Asset::Stock { ticker: "ticker1".to_string() }, 1.0),
        ].iter().cloned().collect();

        let expected: Vec<Transaction> = vec![];

        let actual = balance(&portfolio, &market, &strategy);

        assert_eq!(actual, expected);
    }

    #[test]
    fn portfolio_has_stock_strategy_in_cash() {
        let portfolio = [
            (Asset::Stock { ticker: "ticker1".to_string() }, 15),
        ].iter().cloned().collect();
        let market = [
            (Asset::Stock { ticker: "ticker1".to_string() }, Decimal::new(7825, 2))
        ].iter().cloned().collect();
        let strategy = [
            (Asset::Cash(Decimal::zero()), 1.0)
        ].iter().cloned().collect();

        let expected: Vec<Transaction> = vec![
            (Asset::Stock { ticker: "ticker1".to_string() }, -15)
        ];

        let actual = balance(&portfolio, &market, &strategy);

        assert_eq!(actual, expected);
    }

    #[test]
    fn strategy_has_part_of_portfolio() {
        let portfolio = [
            (Asset::Stock { ticker: "ticker1".to_string() }, 15),
        ].iter().cloned().collect();
        let market = [
            (Asset::Stock { ticker: "ticker1".to_string() }, Decimal::new(7825, 2))
        ].iter().cloned().collect();
        let strategy = [
            (Asset::Stock { ticker: "ticker1".to_string() }, 0.5),
            (Asset::Cash(Decimal::zero()), 0.5)
        ].iter().cloned().collect();

        let expected: Vec<Transaction> = vec![
            (Asset::Stock { ticker: "ticker1".to_string() }, -7)
        ];

        let actual = balance(&portfolio, &market, &strategy);

        assert_eq!(actual, expected);
    }

    #[test]
    fn sold_asset_allow_to_buy_another() {
        let portfolio = [
            (Asset::Stock { ticker: "ticker1".to_string() }, 15),
        ].iter().cloned().collect();
        let market = [
            (Asset::Stock { ticker: "ticker1".to_string() }, Decimal::new(7825, 2)),
            (Asset::Stock { ticker: "ticker2".to_string() }, Decimal::new(24312, 3)),
        ].iter().cloned().collect();
        let strategy = [
            (Asset::Stock { ticker: "ticker1".to_string() }, 0.86),
            (Asset::Stock { ticker: "ticker2".to_string() }, 0.14)
        ].iter().cloned().collect();

        let expected: Vec<Transaction> = vec![
            (Asset::Stock { ticker: "ticker1".to_string() }, -2),
            (Asset::Stock { ticker: "ticker2".to_string() }, 6),
        ];

        let actual = balance(&portfolio, &market, &strategy);

        assert_eq!(actual, expected);
    }

    #[test]
    fn full_sale_one_asset_for_buying_another() {
        let portfolio = [
            (Asset::Stock { ticker: "ticker1".to_string() }, 15),
        ].iter().cloned().collect();
        let market = [
            (Asset::Stock { ticker: "ticker1".to_string() }, Decimal::new(7825, 2)),
            (Asset::Stock { ticker: "ticker2".to_string() }, Decimal::new(24312, 3)),
        ].iter().cloned().collect();
        let strategy = [
            (Asset::Stock { ticker: "ticker2".to_string() }, 1.0)
        ].iter().cloned().collect();

        let expected: Vec<Transaction> = vec![
            (Asset::Stock { ticker: "ticker1".to_string() }, -15),
            (Asset::Stock { ticker: "ticker2".to_string() }, 48),
        ];

        let actual = balance(&portfolio, &market, &strategy);

        assert_eq!(actual, expected);
    }
}

fn get_target_capital(strategy: &Strategy, total: Decimal) -> Capital {
    strategy
        .into_iter()
        .map(|(asset, percentage)| (
            asset.clone(),
            total.mul(Decimal::from_f32(*percentage).unwrap())
        ))
        .collect()
}

#[cfg(test)]
mod target_tests {
    use super::*;

    #[test]
    fn empty_strategy_empty_portfolio() {
        let strategy = HashMap::new();
        let total = Decimal::new(65000, 2);

        let expected: Capital = HashMap::new();

        let actual = get_target_capital(&strategy, total);

        assert_eq!(actual, expected);
    }

    #[test]
    fn strategy_has_one_stock() {
        let strategy = [
            (Asset::Stock { ticker: "ticker1".to_string() }, 1.0),
        ].iter().cloned().collect();
        let total = Decimal::new(65000, 2);

        let expected: Capital = [
            (Asset::Stock { ticker: "ticker1".to_string() }, Decimal::new(65000, 2)),
        ].iter().cloned().collect();

        let actual = get_target_capital(&strategy, total);

        assert_eq!(actual, expected);
    }

    #[test]
    fn strategy_has_two_stock_with_different_parts() {
        let strategy = [
            (Asset::Stock { ticker: "ticker1".to_string() }, 0.62),
            (Asset::Stock { ticker: "ticker2".to_string() }, 0.38),
        ].iter().cloned().collect();
        let total = Decimal::new(783412, 2);

        let expected: Capital = [
            (Asset::Stock { ticker: "ticker1".to_string() }, Decimal::new(48571544, 4)),
            (Asset::Stock { ticker: "ticker2".to_string() }, Decimal::new(29769656, 4)),
        ].iter().cloned().collect();

        let actual = get_target_capital(&strategy, total);

        assert_eq!(actual, expected);
    }

    #[test]
    fn strategy_has_cash() {
        let strategy = [
            (Asset::Stock { ticker: "ticker1".to_string() }, 0.62),
            (Asset::Stock { ticker: "ticker2".to_string() }, 0.37),
            (Asset::Cash(Decimal::zero()), 0.01), //TODO (byTimo) не удобно - не понятно что писать в decimal
        ].iter().cloned().collect();
        let total = Decimal::new(783412, 2);

        let expected: Capital = [
            (Asset::Stock { ticker: "ticker1".to_string() }, Decimal::new(48571544, 4)),
            (Asset::Stock { ticker: "ticker2".to_string() }, Decimal::new(28986244, 4)),
            (Asset::Cash(Decimal::zero()), Decimal::new(783412, 4)),
        ].iter().cloned().collect();

        let actual = get_target_capital(&strategy, total);

        assert_eq!(actual, expected);
    }
}

fn capitalize(portfolio: &Portfolio, market: &Market) -> Capital {
    portfolio.into_iter().filter_map(|(asset, amount)| {
        match asset {
            &Asset::Stock { .. } => {
                let price = market.get(asset)?;
                Some((asset.clone(), price.mul(Decimal::from(*amount))))
            }
            &Asset::Cash(decimal) => {
                Some((asset.clone(), decimal.clone()))
            }
        }
    }).collect()
}

#[cfg(test)]
mod capitalize_tests {
    use super::*;

    #[test]
    fn empty_portfolio_empty_market_empty_capital() {
        let portfolio = HashMap::new();
        let market = HashMap::new();
        let expected: Capital = HashMap::new();

        let actual = capitalize(&portfolio, &market);
        assert_eq!(actual, expected);
    }

    // TODO (byTimo) возможно тут стоит подумать что-то про Option или Result?
    #[test]
    fn market_does_not_have_portfolio_assets_empty_capital() {
        let portfolio = [
            (Asset::Stock { ticker: "ticker1".to_string() }, 5)
        ].iter().cloned().collect();
        let market = [
            (Asset::Stock { ticker: "ticker2".to_string() }, 10.into())
        ].iter().cloned().collect();
        let expected: Capital = HashMap::new();
        let actual = capitalize(&portfolio, &market);
        assert_eq!(actual, expected);
    }

    #[test]
    fn capitalize_assets_in_portfolio_by_market() {
        let portfolio = [
            (Asset::Stock { ticker: "ticker1".to_string() }, 5),
            (Asset::Stock { ticker: "ticker2".to_string() }, 7),
            (Asset::Stock { ticker: "ticker3".to_string() }, 1),
        ].iter().cloned().collect();
        let market = [
            (Asset::Stock { ticker: "ticker1".to_string() }, Decimal::new(3725, 2)),
            (Asset::Stock { ticker: "ticker2".to_string() }, Decimal::new(100324, 2)),
            (Asset::Stock { ticker: "ticker3".to_string() }, Decimal::new(5070, 2)),
        ].iter().cloned().collect();

        let expected: Capital = [
            (Asset::Stock { ticker: "ticker1".to_string() }, Decimal::new(3725 * 5, 2)),
            (Asset::Stock { ticker: "ticker2".to_string() }, Decimal::new(100324 * 7, 2)),
            (Asset::Stock { ticker: "ticker3".to_string() }, Decimal::new(5070, 2)),
        ].iter().cloned().collect();

        let actual = capitalize(&portfolio, &market);
        assert_eq!(actual, expected);
    }

    #[test]
    fn portfolio_has_cash() {
        let portfolio = [
            (Asset::Stock { ticker: "ticker1".to_string() }, 5),
            (Asset::Cash(Decimal::new(65000, 2)), 1),
        ].iter().cloned().collect();
        let market = [
            (Asset::Stock { ticker: "ticker1".to_string() }, Decimal::new(3725, 2)),
        ].iter().cloned().collect();

        let expected: Capital = [
            (Asset::Stock { ticker: "ticker1".to_string() }, Decimal::new(3725 * 5, 2)),
            (Asset::Cash(Decimal::new(65000, 2)), Decimal::new(65000, 2))
        ].iter().cloned().collect();

        let actual = capitalize(&portfolio, &market);
        assert_eq!(actual, expected);
    }
}