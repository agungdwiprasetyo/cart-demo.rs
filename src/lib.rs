pub mod product;
pub mod cart;

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn new_cart() {
        let mut iphone_name = String::from("iphone v");
        let iphone = product::Product::new(1, &mut iphone_name, 8000.0);

        let mut samsung_name = String::from("samsung galaxy 1");
        let samsung = product::Product::new(2, &mut samsung_name, 8000.0);

        let mut item1 = cart::item::Item::new(1, 2, &iphone);
        let mut item2 = cart::item::Item::new(2, 1, &samsung);
        

        let mut items: HashMap<u32, &mut cart::item::Item> = HashMap::new();
        items.insert(item1.id, &mut item1);
        items.insert(item2.id, &mut item2);
        let mut cart1 = cart::Cart::new(1, &mut items);

        assert_eq!(cart1.get_items().len(), 2);

        assert_eq!(cart1.get_total(), 24000.0);

        assert_eq!(cart1.get_item(1).is_some(), true);

    }

    #[test]
    fn add_new_item() {
        let mut items: HashMap<u32, &mut cart::item::Item> = HashMap::new();
        let mut cart1 = cart::Cart::new(1, &mut items);

        let mut nokia_name = String::from("Nokia 6");
        let nokia = product::Product::new(3, &mut nokia_name, 8000.0);
        
        let mut item1 = cart::item::Item::new(3, 5, &nokia);

        cart1.add_item(&mut item1);

        assert_eq!(cart1.get_items().len(), 1);
    }

    #[test]
    fn get_sub_total() {
        let mut nokia_name = String::from("Nokia 6");
        let nokia = product::Product::new(3, &mut nokia_name, 8000.0);
        
        let item1 = cart::item::Item::new(3, 2, &nokia);

        assert_eq!(item1.get_sub_total(), 16000.0);
    }
}
