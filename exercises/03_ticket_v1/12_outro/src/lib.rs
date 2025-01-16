// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: i32,
    unit_price: i32,
}

impl Order {

    pub fn new(product_name: String, quantity: i32, unit_price: i32) -> Order{

        Self::valid_empty(&product_name, "product_name");
        Self::valid_size(&product_name, "product_name");

        Self::valid_value(quantity, "quantity");
        Self::valid_value(unit_price, "unit_price");

        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn set_product_name(&mut self, product_name: String) {
        Self::valid_empty(&product_name, "product_name");
        Self::valid_size(&product_name, "product_name");
        self.product_name = product_name
    }

    pub fn set_quantity(&mut self, quantity: i32) {
        Self::valid_value(quantity, "quantity");
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, unit_price: i32) {
        Self::valid_value(unit_price, "unit_price");
        self.unit_price = unit_price;
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &i32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &i32 {
        &self.unit_price
    }


    pub fn total(&self) -> i32 {
        self.quantity * self.unit_price
    }

    fn valid_empty(param: &String, param_name: &str) {
        if param.is_empty() {
            panic!("{param_name} can't be empty.")
        }
    }


    fn valid_size(param:&String, param_name: &str) {
        if param.capacity() >= 300 {
            panic!("{param_name} cannot be longer than 300 bytes.")
        }
    }

    fn valid_value(param: i32, param_name: &str) {
        if param <= 0 {
            panic!("{param_name} must be strictly greater than zero.")
        }
    }

}