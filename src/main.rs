use std::io;

struct Product {
    name: String,
    price: f32,
}


#[derive (Debug, Copy, Clone)]
struct CoinBox {
    coin_val: f32,
    count: i32,
}

impl CoinBox {
    fn add_coin(& mut self, added_coin: i32){
        self.count += added_coin;
    }

    fn remove_coin(& mut self){
        self.count -= 1;
    }
}

fn main() {
     // display title
    println!("\n*********************************************************************************");
    println!("Welcome to vending/change machine !!");
    println!("**********************************************************************************\n");
   
    let mut coin_box = init_coin_box();
    
    loop {

        let mut products: Vec<Product> = Vec::new();
        

        // create products individually
        products.push(Product{name: "Coke".to_owned(), price: 0.50f32});
        products.push(Product{name: "Pepsi".to_owned(), price: 0.55f32});
        products.push(Product{name: "Soda".to_owned(), price: 0.20f32});
        products.push(Product{name: "Fanta".to_owned(), price: 0.60f32});
        products.push(Product{name: "Oreo".to_owned(), price: 1.50f32});
        products.push(Product{name: "Princles".to_owned(), price: 4.50f32});
        products.push(Product{name: "Lays".to_owned(), price: 3.50f32});
        products.push(Product{name: "Cookies".to_owned(), price: 1.75f32});
        products.push(Product{name: "Sandwich".to_owned(), price: 6.80f32});
        products.push(Product{name: "Choclate".to_owned(), price: 7.10f32});


        let product = display_menu(&products);

        // display price & user guide
        println!("\n****************");
        println!("{} : {}", product.name, product.price);
        println!("****************\n");
        
        let user_inserted = insert_coins(&mut coin_box, product.price);

        if user_inserted > product.price {
            coin_box = calculate_exchange( coin_box, user_inserted - product.price);
        }
        
    }// end of main loop
}
    


fn display_menu(products: &Vec<Product>) -> &Product{
    let mut menu_num = 1;
    for p in products.iter(){
        println!("{}. {}: {}", menu_num, p.name, p.price);
        menu_num += 1;
    }

    let item_index = get_user_menu_choice() - 1 as usize;
    &products[item_index]
}


fn get_user_menu_choice() -> usize{

    loop {
        let mut user_input: String = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if user_input < 1 || user_input > 10{
            println!("Please enter number corresponding to menu items! (1-10)");
        }else {
            return user_input as usize;
        }
        
    }
}


fn init_coin_box() -> Vec<CoinBox> {

    let mut coin_boxes: Vec<CoinBox> = Vec::new();

    coin_boxes.push(CoinBox { coin_val: 2.00f32, count: 20 });
    coin_boxes.push(CoinBox { coin_val: 1.00f32, count: 20 });
    coin_boxes.push(CoinBox { coin_val: 0.50f32, count: 20 });
    coin_boxes.push(CoinBox { coin_val: 0.20f32, count: 20 });
    coin_boxes.push(CoinBox { coin_val: 0.10f32, count: 20 });
    coin_boxes.push(CoinBox { coin_val: 0.05f32, count: 20 });
    coin_boxes.push(CoinBox { coin_val: 0.02f32, count: 20 });
    coin_boxes.push(CoinBox { coin_val: 0.01f32, count: 20 });
    
    coin_boxes
}


fn insert_coins(v_coin_box: & mut Vec<CoinBox>, price: f32) -> f32{
    let mut total_amount_entered: f32 = 0.00f32;

    loop {
        // present coin options
        println!("\n** Insert coins using following menu: (1-8)");
        println!("1. 2.00 coins");
        println!("2. 1.00 coins");
        println!("3. 0.50 coins");
        println!("4. 0.20 coins");
        println!("5. 0.10 coins");
        println!("6. 0.05 coins");
        println!("7. 0.02 coins");
        println!("8. 0.01 coins");
        println!("\n0. Complete transaction");

        let mut user_input: String = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        

        let user_input: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if user_input == 0 {
            if total_amount_entered < price{
                println!("You haven't entered enough coins yet!");
                continue;
            }else {
                return total_amount_entered;
            }
        }

        match user_input {
            1 => {
                    if v_coin_box[0].count >= 50 {
                        println!("Can't accept anymore {} coins", v_coin_box[0].coin_val);
                    }else {
                        v_coin_box[0].add_coin(1);
                        total_amount_entered += v_coin_box[0].coin_val;
                        println!("Total inserted: {} coins", total_amount_entered);
                    }
                },
            2 => {
                    if v_coin_box[1].count >= 50 {
                        println!("Can't accept anymore {} coins", v_coin_box[1].coin_val);
                    }else {
                        v_coin_box[1].add_coin(1);
                        total_amount_entered += v_coin_box[1].coin_val;
                        println!("Total inserted: {} coins", total_amount_entered);
                    }
                },
            3 => {
                    if v_coin_box[2].count >= 50 {
                        println!("Can't accept anymore {} coins", v_coin_box[2].coin_val);
                    }else {
                        v_coin_box[2].add_coin(1);
                        total_amount_entered += v_coin_box[2].coin_val;
                        println!("Total inserted: {} coins", total_amount_entered);
                    }
                },
            4 => {
                    if v_coin_box[3].count >= 50 {
                        println!("Can't accept anymore {} coins", v_coin_box[3].coin_val);
                    }else {
                        v_coin_box[3].add_coin(1);
                        total_amount_entered += v_coin_box[3].coin_val;
                        println!("Total inserted: {} coins", total_amount_entered);
                    }
                },
            5 => {
                    if v_coin_box[4].count >= 50 {
                        println!("Can't accept anymore {} coins", v_coin_box[4].coin_val);
                    }else {
                        v_coin_box[4].add_coin(1);
                        total_amount_entered += v_coin_box[4].coin_val;
                        println!("Total inserted: {} coins", total_amount_entered);
                    }
                },
            6 => {
                    if v_coin_box[5].count >= 50 {
                        println!("Can't accept anymore {} coins", v_coin_box[5].coin_val);
                    }else {
                        v_coin_box[5].add_coin(1);
                        total_amount_entered += v_coin_box[5].coin_val;
                        println!("Total inserted: {} coins", total_amount_entered);
                    }
                },
            7 => {
                    if v_coin_box[6].count >= 50 {
                        println!("Can't accept anymore {} coins", v_coin_box[6].coin_val);
                    }else {
                        v_coin_box[6].add_coin(1);
                        total_amount_entered += v_coin_box[6].coin_val;
                        println!("Total inserted: {} coins", total_amount_entered);
                    }
                },
            8 => {
                    if v_coin_box[7].count >= 50 {
                        println!("Can't accept anymore {} coins", v_coin_box[7].coin_val);
                    }else {
                        v_coin_box[7].add_coin(1);
                        total_amount_entered += v_coin_box[7].coin_val;
                        println!("Total inserted: {} coins", total_amount_entered);
                    }
                },
            _ => println!("Something went wrong..."),

        };
    }
}


fn calculate_exchange(coin_box: Vec<CoinBox>, difference: f32)-> Vec<CoinBox>{

    let v_coin_box = coin_box.clone();

    let mut change_left = difference;
    
    println!("Change to Give {}", difference);

    for c in v_coin_box.iter(){
        while change_left / c.coin_val > 1.0f32 {

            if c.count >= 1 {
                change_left -= c.coin_val;
                {*c}.count -= 1;
                {*c}.remove_coin();
                
            }
        }
    }
    if change_left < 0.0009f32{
        println!("Transaction Successful");
        return v_coin_box;
    }else{
        return coin_box;
    }

}
