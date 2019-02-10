use std::collections::HashMap;

fn main() {
    let mut h: HashMap<u32, &mut Item> = HashMap::new();

    let mut iphone_name = String::from("iphone v");
    let iphone = Product{id: 1, name: &mut iphone_name, price: 8000.0};

    let mut samsung_name = String::from("samsung galaxy 1");
    let samsung = Product{id: 2, name: &mut samsung_name, price: 8000.0};

    let mut nokia_name = String::from("Nokia 6");
    let nokia = Product{id: 3, name: &mut nokia_name, price: 8000.0};

    let mut a = Item::new(1, 2, &iphone);
    let mut b = Item::new(2, 1, &samsung);
    let mut c = Item::new(3, 5, &nokia);

    h.insert(1, &mut a);
    h.insert(2, &mut b);

    if let Some(v) = h.get_mut(&1) {
        *v = &mut c;
        //*v.update(3, p);
    }

    for (key, value) in &h {
        println!("{}: {:?}", key, value);
    }
}

#[derive(Debug)]
struct Product<'a> {
    id: u32,
    name: &'a mut String,
    price: f32,
}

#[derive(Debug)]
struct Item {
    id: u32,
    qty: u32,
    sub_total: f32,
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