use super::cart;

#[derive(Debug)]
pub enum Status {
    Created(String),
    Unpaid(String),
    Paid(String),
}

#[derive(Debug)]
pub struct Billing {
    pub name:  String,
    pub address: String,
    pub shipping_cost: f32,
}

#[derive(Debug)]
pub struct Order<'c, 'a, 'b: 'a, 's: 'b, 'd> {
    pub id: u32,
    pub cart: &'c mut cart::Cart<'a, 'b, 's>,
    pub billing: &'d mut Billing,
    pub status: Status,
}

impl<'c, 'a, 'b, 's, 'd> Order<'c, 'a, 'b, 's, 'd> {
    pub fn new(id: u32, cart: &'c mut cart::Cart<'a, 'b, 's>, billing: &'d mut Billing) -> Order<'c, 'a, 'b, 's, 'd> {
        Order{
            id,
            cart,
            billing,
            status: Status::Created(String::from("CREATED")),
        }
    }

    pub fn get_total(&self) -> f32 {
        self.cart.get_total() + self.billing.shipping_cost
    }
}