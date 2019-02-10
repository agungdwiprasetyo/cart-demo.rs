use std::collections::HashMap;

pub struct Product<'a> {
    pub id: u32,
    pub name: &'a mut String,
    pub price: f32,
}

pub struct Item {
    pub id: u32,
    pub qty: u32,
    sub_total: f32,
}

pub struct Cart<'a, 'b: 'a> {
    pub id: u32,
    items: &'a mut HashMap<u32, &'b mut Item>,
}

impl<'a, 'b>Cart<'a, 'b> {
    pub fn new(id: u32, items: &'a mut HashMap<u32, &'b mut Item>) -> Cart<'a, 'b>{
        Cart{
            id: id,
            items: items,
        }
    }

    pub fn add_item(&mut self, item: &'b mut Item) {
        self.items.insert(item.id, item);
    }

    pub fn update_quantity(&mut self, id: u32, qty: u32, product: &Product){
        if let Some(item) = self.get_item(id) {
            item.update(qty, &product);
        }
    }

    pub fn get_item(&mut self, id: u32) -> Option<&mut Item> {
        match self.items.get_mut(&id) {
            Some(item) => Some(item),
            None => None
        }
    }

    pub fn get_items(&self) -> &HashMap<u32, &'b mut Item> {
        self.items
    }

    pub fn get_total(&self) -> f32 {
        let mut total: f32 = 0.0;
        for (_key, value) in self.items.iter() {
            total = total + value.sub_total;
        }
        total
    }
}

impl Item {
    pub fn new(id: u32, qty: u32, product: &Product) -> Item {
        let sub_total = qty as f32 * product.price;
        Item{
            id,
            qty,
            sub_total,
        }
    }

    pub fn update(&mut self, qty: u32, product: &Product) {
        self.qty = qty;
        self.sub_total = qty as f32 * product.price;
    }

    pub fn get_sub_total(&self) -> f32 {
        self.sub_total
    }
}

impl<'a> Product<'a> {
    pub fn new(id: u32, name: &'a mut String, price: f32) -> Product{
        Product{
            id,
            name,
            price,
        }
    }
}