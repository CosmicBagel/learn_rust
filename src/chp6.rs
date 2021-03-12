#[derive(Debug, Clone, Copy)]
enum Province {
    AB,
    BC,
    QC
}

#[derive(Clone, Copy)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Province),
}


fn main () {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    plus_one(Some(5));
    let coin = Coin::Dime;
    let mut count = 0;

    // println!("fsafsfds {:?}", )

    if let Coin::Quarter(prov) = coin {
        println!("province quarter from {:?}!", prov);
    } else {
        count += 1;
    };

    match coin {
        Coin::Quarter(prov) => {
            println!("Prov quarter from {:?}!", prov);
        }
        _ => {
            count += 1;
        }
    };
}
