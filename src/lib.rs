pub mod product;
pub mod cart;
pub mod order;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use super::cart::item;

    #[test]
    fn create_order() {
        

        let mut iphone_name = String::from("iphone v");
        let mut iphone = product::Product::new(1, &mut iphone_name, 8000.0);

        let mut samsung_name = String::from("samsung galaxy 1");
        let mut samsung = product::Product::new(2, &mut samsung_name, 8000.0);

        let mut item1 = item::Item::new(1, 2, &mut iphone);
        let mut item2 = item::Item::new(2, 1, &mut samsung);
        

        let mut items: HashMap<u32, &mut item::Item> = HashMap::new();
        items.insert(item1.id, &mut item1);
        items.insert(item2.id, &mut item2);
        let mut cart1 = cart::Cart::new(1, &mut items);

        let mut billing = order::Billing{
            name: String::from("Wuriyanto"),
            address: String::from("Banjarnegara"),
            shipping_cost: 5000.0,
        };

        let order1 = order::Order::new(1, &mut cart1, &mut billing);

        assert_eq!(order1.get_total(), 29000.0);
    }
}