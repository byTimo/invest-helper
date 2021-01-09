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
mod tests {
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