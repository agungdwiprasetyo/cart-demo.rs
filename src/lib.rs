pub mod cart;

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn new_cart() {
        let mut iphone_name = String::from("iphone v");
        let iphone = cart::product::Product::new(1, &mut iphone_name, 8000.0);

        let mut samsung_name = String::from("samsung galaxy 1");
        let samsung = cart::product::Product::new(2, &mut samsung_name, 8000.0);

        let mut item1 = cart::Item::new(1, 2, &iphone);
        let mut item2 = cart::Item::new(2, 1, &samsung);
        

        let mut items: HashMap<u32, &mut cart::Item> = HashMap::new();
        items.insert(item1.id, &mut item1);
        items.insert(item2.id, &mut item2);
        let cart1 = cart::Cart::new(1, &mut items);

        assert_eq!(cart1.get_items().len(), 2);

    }

    #[test]
    fn add_new_item() {
        let mut items: HashMap<u32, &mut cart::Item> = HashMap::new();
        let mut cart1 = cart::Cart::new(1, &mut items);

        let mut nokia_name = String::from("Nokia 6");
        let nokia = cart::product::Product::new(3, &mut nokia_name, 8000.0);
        
        let mut item1 = cart::Item::new(3, 5, &nokia);

        cart1.add_item(&mut item1);

         assert_eq!(cart1.get_items().len(), 1);
    }
}
