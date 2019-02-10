use super::product;

#[derive(Debug)]
pub struct Item<'a, 's: 'a> {
    pub id: u32,
    pub product: &'a product::Product<'s>,
    pub qty: u32,
    pub sub_total: f32,
}

impl<'a, 's> Item<'a, 's> {
    pub fn new(id: u32, qty: u32, product: &'a product::Product<'s>) -> Item<'a, 's> {
        let sub_total = qty as f32 * product.price;
        Item{
            id,
            product,
            qty,
            sub_total,
        }
    }

    pub fn update(&mut self, qty: u32, product: &'a product::Product<'s>) {
        self.qty = qty;
        self.product = product;
        self.sub_total = qty as f32 * product.price;
    }

    pub fn get_sub_total(&self) -> f32 {
        self.sub_total
    }
}