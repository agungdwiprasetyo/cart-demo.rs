use super::product;

#[derive(Debug)]
pub struct Item {
    pub id: u32,
    pub qty: u32,
    pub sub_total: f32,
}

impl Item {
    pub fn new(id: u32, qty: u32, product: &product::Product) -> Item {
        let sub_total = qty as f32 * product.price;
        Item{
            id,
            qty,
            sub_total,
        }
    }

    pub fn update(&mut self, qty: u32, product: &product::Product) {
        self.qty = qty;
        self.sub_total = qty as f32 * product.price;
    }

    pub fn get_sub_total(&self) -> f32 {
        self.sub_total
    }
}