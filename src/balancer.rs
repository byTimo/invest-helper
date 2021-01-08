use rust_decimal::prelude::*;
use std::collections::HashMap;
use std::ops::Mul;

#[derive(Hash, Debug, Eq, PartialEq, Clone)]
enum Asset {
    Stock { ticker: String },
    Cash,
}

fn capitalize(portfolio: &HashMap<Asset, u64>, market: &HashMap<Asset, Decimal>) -> HashMap<Asset, Decimal> {
    portfolio.into_iter().filter_map(|(asset, amount)| {
        let price = market.get(&asset)?;
        Some((asset.clone(), price.mul(Decimal::from(*amount))))
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_portfolio__empty_market__empty_capital() {
        let portfolio = HashMap::new();
        let market = HashMap::new();
        let expected: HashMap<Asset, Decimal> = HashMap::new();

        let actual = capitalize(&portfolio, &market);
        assert_eq!(actual, expected);
    }

    // TODO (byTimo) возможно тут стоит подумать что-то про Option или Result?
    #[test]
    fn market_does_not_have_portfolio_assets__empty_capital() {
        let portfolio = [
            (Asset::Stock { ticker: "ticker1".to_string() }, 5)
        ].iter().cloned().collect();
        let market = [
            (Asset::Stock { ticker: "ticker2".to_string() }, 10.into())
        ].iter().cloned().collect();
        let expected: HashMap<Asset, Decimal> = HashMap::new();
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

        let expected: HashMap<Asset, Decimal> = [
            (Asset::Stock { ticker: "ticker1".to_string() }, Decimal::new(3725 * 5, 2)),
            (Asset::Stock { ticker: "ticker2".to_string() }, Decimal::new(100324 * 7, 2)),
            (Asset::Stock { ticker: "ticker3".to_string() }, Decimal::new(5070, 2)),
        ].iter().cloned().collect();

        let actual = capitalize(&portfolio, &market);
        assert_eq!(actual, expected);
    }
}