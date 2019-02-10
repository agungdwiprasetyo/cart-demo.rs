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

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn new_cart() {
        let mut iphone_name = String::from("iphone v");
        let mut iphone = product::Product::new(1, &mut iphone_name, 8000.0);

        let mut samsung_name = String::from("samsung galaxy 1");
        let mut samsung = product::Product::new(2, &mut samsung_name, 8000.0);

        let mut item1 = item::Item::new(1, 2, &mut iphone);
        let mut item2 = item::Item::new(2, 1, &mut samsung);
        

        let mut items: HashMap<u32, &mut item::Item> = HashMap::new();
        items.insert(item1.id, &mut item1);
        items.insert(item2.id, &mut item2);
        let cart1 = Cart::new(1, &mut items);

        assert_eq!(cart1.get_items().len(), 2);

        assert_eq!(cart1.get_total(), 24000.0);

        //assert_eq!(cart1.get_item(1).is_some(), true);

    }

    #[test]
    fn get_cart_item() {
        let mut iphone_name = String::from("iphone v");
        let mut iphone = product::Product::new(1, &mut iphone_name, 8000.0);

        let mut samsung_name = String::from("samsung galaxy 1");
        let mut samsung = product::Product::new(2, &mut samsung_name, 8000.0);

        let mut item1 = item::Item::new(1, 2, &mut iphone);
        let mut item2 = item::Item::new(2, 1, &mut samsung);
        

        let mut items: HashMap<u32, &mut item::Item> = HashMap::new();
        items.insert(item1.id, &mut item1);
        items.insert(item2.id, &mut item2);
        let mut cart1 = Cart::new(1, &mut items);

        assert_eq!(cart1.get_item(1).is_some(), true);
    }

    #[test]
    fn add_new_item() {
        let mut items: HashMap<u32, &mut item::Item> = HashMap::new();
        let mut cart1 = Cart::new(1, &mut items);

        let mut nokia_name = String::from("Nokia 6");
        let mut nokia = product::Product::new(3, &mut nokia_name, 8000.0);
        
        let mut item1 = item::Item::new(3, 5, &mut nokia);

        cart1.add_item(&mut item1);

        assert_eq!(cart1.get_items().len(), 1);

    }

     #[test]
    fn update_item() {
        let mut items: HashMap<u32, &mut item::Item> = HashMap::new();
        let mut cart1 = Cart::new(1, &mut items);

        let mut nokia_name = String::from("Nokia 6");
        let mut nokia = product::Product::new(3, &mut nokia_name, 8000.0);
        
        let mut item1 = item::Item::new(3, 5, &mut nokia);

        cart1.add_item(&mut item1);

        let mut nokia_name_update = String::from("Nokia 6");
        let nokia_update = product::Product::new(3, &mut nokia_name_update, 8000.0);

        cart1.update_item(3, 4, &nokia_update);

        assert_eq!(cart1.get_total(), 32000.0);
    }

    #[test]
    fn get_sub_total() {
        let mut nokia_name = String::from("Nokia 6");
        let mut nokia = product::Product::new(3, &mut nokia_name, 8000.0);
        
        let item1 = item::Item::new(3, 2, &mut nokia);

        assert_eq!(item1.get_sub_total(), 16000.0);
    }
}