struct Solution{

}

impl Solution {

    fn count_coins(coins: Vec<i32>, amount: i32, count: i32) -> i32 {

        for coin in coins {

            if amount == coin {
                return count + 1;
            } else if amount > coin {
                return count_coins(coins, amount - coin, count + 1);
            } else {
                return -1;
            }
        }       

        return -1;

    }

    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        
        coins.sort_by(|a, b| b.cmp(a));

        let count_coins = |amount: i32, count: i32| {

            for coin in coins {
    
                if amount == coin {
                    return count + 1;
                } else if amount > coin {
                    return count_coins(coins, amount - coin, count + 1);
                } else {
                    return -1;
                }
            }       
    
            return -1;
    
        };

        Solution::count_coins(coins, amount, 0)
    }
}

fn main() {

}