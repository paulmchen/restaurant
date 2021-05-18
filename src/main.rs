use restaurant::{
    eat_at_restaurant, plus_one, route, value_in_cents, Coin, IpAddrKind, Message, Rectangle,
    UsState,
};

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    route(home);
    route(loopback);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let m = Message::Write(String::from("hello"));
    m.call();

    let v = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("The coin value is {}.", v);

    let seven = Some(5);
    let _eight = plus_one(seven);
    let _none = plus_one(None);

    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rec2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rec3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rec1 hold rect2? {}.", rec1.can_hold(&rec2));
    println!("Can rec1 hold rect2? {}.", rec1.can_hold(&rec3));

    let rec4 = Rectangle::square(3);
    println!("Can rec1 hold rec4? {}.", rec1.can_hold(&rec4));

    // call eat_at_rest lib function
    eat_at_restaurant();
}
