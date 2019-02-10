use std::collections::HashMap;
pub mod item;
use super::product;

#[derive(Debug)]
pub struct Cart<'a, 'b: 'a, 's: 'b> {
    pub id: u32,
    items: &'a mut HashMap<u32, &'b mut item::Item<'b, 's>>,
}

impl<'a, 'b, 's>Cart<'a, 'b, 's> {
    pub fn new(id: u32, items: &'a mut HashMap<u32, &'b mut item::Item<'b, 's>>) -> Cart<'a, 'b, 's>{
        Cart{
            id: id,
            items: items,
        }
    }

    pub fn add_item(&mut self, item: &'b mut item::Item<'b, 's>) {
        self.items.insert(item.id, item);
    }

    pub fn update_item(&mut self, id: u32, qty: u32, product: &'b product::Product<'s>){
        if let Some(item) = self.get_item(id) {
            item.update(qty, &product);
        }
    }

    pub fn get_item(&mut self, id: u32) -> Option<&mut item::Item<'b, 's>> {
        match self.items.get_mut(&id) {
            Some(item) => Some(item),
            None => None
        }
    }

    pub fn get_items(&self) -> &HashMap<u32, &'b mut item::Item<'b, 's>> {
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