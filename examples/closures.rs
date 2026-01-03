use core::num;

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Green
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    fn giveaway(&self, preference:Option<ShirtColor>)-> ShirtColor {
        preference.unwrap_or_else(|| self.most_stacked())
    }

    fn most_stacked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_green = 0;

        for color in self.shirts.iter() {
            match color {
                ShirtColor::Green => num_green += 1,
                ShirtColor::Red => num_red += 1
            };
        }

        if num_red > num_green {
            ShirtColor::Green
        } else {
            ShirtColor::Green
        }
    }
}



fn main1() {
    let inv = Inventory {
        shirts: vec![ShirtColor::Green, ShirtColor::Red, ShirtColor::Green, ShirtColor::Red, ShirtColor::Green]
    };

    let user_color = Some(ShirtColor::Green);
    let giveaway_color = inv.giveaway(user_color);
    println!("User with {user_color:?} color preference got {giveaway_color:?} shirt");
    }

fn main2() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // FnOnce applies to closures that can be called once.

    // FnMut applies to closures that don’t move captured values out of their body
    // but might mutate the captured values.

    // Fn applies to closures that don’t move captured values out of their body 
    // and don’t mutate captured values, 
    // as well as closures that capture nothing from their environment.

    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    borrows_mutably();
    println!("After calling closure: {list:?}");
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let mut value = String::from("closure called");

    list.sort_by_key(|r| {
        sort_operations.push("yo");
        value.push_str("1");
        r.width
    });
    println!("{list:#?}");
}