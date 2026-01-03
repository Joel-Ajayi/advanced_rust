fn largest<T>(list:&[T]) -> &T
where T: PartialOrd
{
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main1() {
    let num_list  = vec![34,50, 25, 100, 65];

    let res = largest(&num_list);
    println!("The largest number is {res}");

    let num_list  = vec!['y', 'm', 'a', 'q'];
    let res = largest(&num_list);
    println!("The largest number is {res}");
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0 as f32, y: 10.0 as f32 };

    println!("p.x = {}", p.x());
}