#[derive(Debug)]
pub struct Product<'a> {
    pub id: u32,
    pub name: &'a mut String,
    pub price: f32,
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