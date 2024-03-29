fn main() {
    println!("Hello, world!");
    let _two = 2; // integer
    let mut _my_name = "Bill"; // string

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let tri = add(5, 7);

    println!("{:?}", tri);

    // enum
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    fn which_way(go: Direction) -> &'static str {
        match go {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
            _ => "omni",
        }
    }

    // structs for grouping similar data
    struct GroceryItem {
        stock: i32,
        price: f64,
    }

    let cereal = GroceryItem {
        stock: 10,
        price: 3.99,
    };

    println!("cereal stock: {:?}", cereal.stock);
    println!("cereal price: {:?}", cereal.price);

    // tuples are records, they are usually used to group results when a fn is to return a set of data
    fn one_two_three() -> (i32, i32, i32) {
        (1, 2, 3)
    }

    let numbers = one_two_three();
    let (x, y, z) = one_two_three();
    println!("{:?}, {:?}", x, numbers.0);
    println!("{:?}, {:?}", y, numbers.1);

    let (employee, direction) = ("Jake", Direction::Right);
    println!("{:?}", employee);

    // vectors are arrays
    let my_nums = vec![1, 2, 3];
    let mut my_nums = Vec::new();
    my_nums.push(4);
    my_nums.push(5);
    my_nums.push(6);
    println!("{:?}", my_nums);
    my_nums.pop();
    let length = my_nums.len();
    println!("{:?}", length);

    let idx = my_nums[1];
    println!("{:?}", idx);

    for num in my_nums {
        println!("{:?}", num);
    }
}
