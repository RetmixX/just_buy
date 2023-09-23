use crate::DBPool;
use crate::product::dto::{ProductDto, UpsertProductDto};
use crate::product::repository::product_data::ProductData;
use crate::shared::error_handler::ApiError;

pub struct ProductService {
    product_repository: ProductData,
}

impl ProductService {
    pub fn new(db: DBPool) -> Self {
        Self {
            product_repository: ProductData::new(db)
        }
    }
}

impl ProductService {
    pub async fn find_all_products(&self) -> Result<Vec<ProductDto>, ApiError> {
        let result = self.product_repository.all_products().await
            ?.iter().map(|product|
            ProductDto::from(product)).collect();

        Ok(result)
    }

    pub async fn find_product(&self, id: &i32) -> Result<ProductDto, ApiError> {
        let result =
            ProductDto::from(&self.product_repository.get_product(id).await?);

        Ok(result)
    }

    pub async fn update_product(&self, id: &i32, data: &UpsertProductDto) -> Result<ProductDto, ApiError> {
        let result = self.product_repository.update_product(id, data).await?;
        Ok(ProductDto::from(&result))
    }

    pub async fn create_product(&self, data: &UpsertProductDto) -> Result<ProductDto, ApiError> {
        let result = self.product_repository.create_product(data).await?;

        Ok(ProductDto::from(&result))
    }

    pub async fn delete_product(&self, id: &i32) -> Result<(), ApiError> {
        self.product_repository.delete_product(id).await
    }
}