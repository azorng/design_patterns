trait QuackBehavior {
    fn quack(&self);
}

struct NormalQuack;
impl QuackBehavior for NormalQuack {
    fn quack(&self) {
        println!("QUAAAACK")
    }
}

struct NoQuack;
impl QuackBehavior for NoQuack {
    fn quack(&self) {
        println!("Wish I could quack but nope")
    }
}

trait FlyBehavior {
    fn fly(&self);
}

struct FlyWithWings;
impl FlyBehavior for FlyWithWings {
    fn fly(&self) {
        println!("Flyinnnnnnn like an angel")
    }
}

struct NoFly;
impl FlyBehavior for NoFly {
    fn fly(&self) {
        println!("Wish I could flyyy")
    }
}

enum DuckType {
    RubberDuck,
    MallardDuck,
}

struct Duck {
    quack_behavior: Box<dyn QuackBehavior>,
    fly_behavior: Box<dyn FlyBehavior>,
    display_behavior: Box<dyn Fn() -> ()>,
}
impl Duck {
    fn quack(&self) {
        self.quack_behavior.quack();
    }

    fn fly(&self) {
        self.fly_behavior.fly();
    }

    fn swim(&self) {
        println!("To swim is not a problem")
    }

    fn display(&self) {
        (*self.display_behavior)()
    }

    fn new(duck_type: DuckType) -> Self {
        match duck_type {
            DuckType::RubberDuck => Duck {
                quack_behavior: Box::new(NoQuack),
                fly_behavior: Box::new(NoFly),
                display_behavior: Box::new(|| println!("RUBBER DUCKS ROCK IT LIKE THISSS")),
            },
            DuckType::MallardDuck => Duck {
                quack_behavior: Box::new(NormalQuack),
                fly_behavior: Box::new(FlyWithWings),
                display_behavior: Box::new(|| println!("BEHOLD, THE MIGHTY MALLARD DUCK")),
            },
        }
    }
}

fn main() {
    let mallard_duck = Duck::new(DuckType::MallardDuck);
    let rubber_duck = Duck::new(DuckType::RubberDuck);

    for duck in [mallard_duck, rubber_duck] {
        duck.display();
        duck.quack();
        duck.fly();
        duck.swim();
    }
}
