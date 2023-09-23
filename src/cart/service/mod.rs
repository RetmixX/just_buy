use crate::cart::dto::CartUserDto;
use crate::cart::repository::cart_data::CartData;
use crate::DBPool;
use crate::shared::error_handler::ApiError;

pub struct CartService {
    cart_data: CartData,
}

impl CartService {
    pub fn new(db: DBPool) -> Self {
        Self {
            cart_data: CartData::new(db)
        }
    }
}

impl CartService {
    pub async fn user_cart(&self, user_id: &i32) -> Result<Vec<CartUserDto>, ApiError> {
        let result =
            self.cart_data
                .show_user_cart(user_id).await?.iter()
                .map(|cart| CartUserDto::from(cart)).collect();

        Ok(result)
    }

    pub async fn add_product_to_cart(&self, user_id: &i32, product_id: &i32) -> Result<(), ApiError> {
        self.cart_data.add_product_to_cart(user_id, product_id).await
    }

    pub async fn remove_product_from_user(&self, user_id: &i32, position_product: &i32)
        -> Result<(), ApiError> {
        self.cart_data.remove_product_from_cart(user_id, position_product).await
    }
}
