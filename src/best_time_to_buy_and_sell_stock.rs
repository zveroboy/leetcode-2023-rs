/**
* 121. Best Time to Buy and Sell Stock
*/
struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut max_profit = 0;

        for price in prices {
            max_profit = max_profit.max(price - min_price);
            min_price = min_price.min(price);
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // #[ignore]
    #[test]
    fn test_case_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];

        let res = Solution::max_profit(prices);

        assert_eq!(res, 5);
    }

    #[ignore]
    #[test]
    fn test_case_2() {
        let prices = vec![7, 6, 4, 3, 1];

        let res = Solution::max_profit(prices);

        assert_eq!(res, 0);
    }
}
