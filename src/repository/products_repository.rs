use crate::domain::product::Product;
use crate::repository::{Identifiable, Repository};

pub type ProductsRepository = Repository<Product>;

// видимость методов задается из trait, а тут просто реализуем
impl Identifiable for Product {
    fn get_id(&self) -> String {
        self.get_article().to_string()
    }
}
