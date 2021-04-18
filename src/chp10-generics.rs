struct Point<T> {
    x: T,
    y: T,
}

impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointMixed<T, U> {
    x: T,
    y: U,
}

impl <T, U> PointMixed<T, U> {
    fn mixup<V, W>(self, other: PointMixed<V, W>) -> PointMixed<T, W> {
        PointMixed {
            x: self.x,
            y: other.y,
        }
    }
}


fn main () {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let pm1 = PointMixed { x: 5, y: 10.4 };
    let pm2 = PointMixed { x: "Hello", y: 'c' };

    let pm3 = pm1.mixup(pm2);
    println!("pm3.x = {}, pm3.y = {}", pm3.x, pm3.y);
}