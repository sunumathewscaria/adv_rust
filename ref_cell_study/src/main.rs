use std::{cell::{RefCell, Cell}, rc::Rc};



fn main() {
    println!("Hello, world!");

    let mut order1 =  Order{product_name: "phone".to_string(), product_id: (1)};
    println!("{:?}", order1);
    let mut test_cust = Customer{
        customer_id: 1,
        customer_name: "test_cust".to_string(),
        orders: vec![]
    };
    test_cust.orders.push( order1);
    println!("{:?}", test_cust);

    change_product_id(&mut test_cust);

    println!("{:?}", test_cust);


}

fn change_product_id(test_cust:&mut Customer){
    test_cust.orders.get_mut(0).unwrap().product_id =2;
}


#[derive(Debug)]
pub struct Customer{
    customer_id: u8,
    customer_name: String,
    orders: Vec<Order>
}

#[derive(Debug)]
pub struct Order{
    product_name:String,
    product_id: u8,
}
