#[derive(Debug)]
pub struct Product<'s> {
    pub id: u32,
    pub name: &'s mut String,
    pub price: f32,
}


impl<'s> Product<'s> {
    pub fn new(id: u32, name: &'s mut String, price: f32) -> Product{
        Product{
            id,
            name,
            price,
        }
    }
}