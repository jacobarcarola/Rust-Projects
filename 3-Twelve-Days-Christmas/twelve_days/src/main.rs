fn main() {
    let mut n: usize = 1;
    while n <= 12 {
        print_day(n);
        print_gifts(n);
        n = n + 1;
    }
}

fn print_day(n: usize) {
    if n == 1 || n == 2 || n == 3 {
        if n == 1 {
            println!("On the {n}st day of Christmas my true love gave to me");
        } else if n == 2 {
            println!("On the {n}nd day of Christmas my true love gave to me");
        } else if n == 3 {
            println!("On the {n}rd day of Christmas my true love gave to me");
        }
    } else {
        println!("On the {n}th day of Christmas my true love gave to me");
    }
}

fn print_gifts(day: usize) {
    let day_one = String::from("A partridge in a pear tree.");
    let one = String::from("And a partridge in a pear tree.");
    let two = String::from("Two turtle doves,");
    let three = String::from("Three French hens,");
    let four = String::from("Four calling birds,");
    let five = String::from("Five gold rings,");
    let six = String::from("Six geese a-laying,");
    let seven = String::from("Seven swans a-swimming,");
    let eight = String::from("Eight maids a-milking,");
    let nine = String::from("Nine ladies dancing,");
    let ten = String::from("Ten lords a-leaping,");
    let eleven = String::from("Eleven pipers piping,");
    let twelve = String::from("Twelve drummers drumming,");

    let a: [String; 12] = [one, two, three,
                            four, five, six,
                             seven, eight, nine,
                              ten, eleven, twelve];

    let mut i: usize = day;
    while i >= 1 {
        if day == 1 {
            println!("{}", day_one);
            i = i - 1;
        } else {
            println!("{}", a[i - 1]);
            i = i - 1;
        }
    }
    println!("");
}