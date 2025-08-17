#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    // expected public fields
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Default::default(),
            receipt: Default::default(),
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        let pos = s.products.iter().position(|(x, _)| *x == ele).unwrap();
        self.items.push((ele, s.products[pos].1));
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let total_original: f32 = prices.iter().sum();
        let mut total_discount = 0.0;

        let groups = prices.len() / 3;
        for i in 0..groups {
            total_discount += prices[i];
        }

        let total_after_discount = total_original - total_discount;

        let ratio = total_after_discount / total_original;

        let mut adjusted_prices: Vec<f32> = self
            .items
            .iter()
            .map(|(_, price)| {
                let adjusted = price * ratio;
                (adjusted * 100.0).round() / 100.0
            })
            .collect();

        adjusted_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        self.receipt = adjusted_prices.clone();
        adjusted_prices
    }
}
