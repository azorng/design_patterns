interface Flyable {
    fly(): void
}

class FlyWithWings implements Flyable {
    fly() {
        console.log('I am flying with wings')
    }
}

class NoFly implements Flyable {
    fly() {
        console.log('I can not fly :(')
    }
}

interface Quackable {
    quack(): void
}

class Quack implements Quackable {
    quack() {
        console.log('QUAAACK')
    }
}

class NoQuack implements Quackable {
    quack() {
        console.log('I can not quack :(')
    }
}

abstract class Duck {
    flyBehavior: Flyable
    quackBehavior: Quackable
    abstract display(): void
    swim() {
        console.log('All ducks can swim!')
    }

    fly() {
        this.flyBehavior.fly()
    }

    quack() {
        this.quackBehavior.quack()
    }
}

class RubberDuck extends Duck {
    constructor() {
        super()
        this.quackBehavior = new NoQuack()
        this.flyBehavior = new NoFly()
    }

    display() {
        console.log('Displays a rubber duck')
    }
}

class MallardDuck extends Duck {
    constructor() {
        super()
        this.quackBehavior = new Quack()
        this.flyBehavior = new FlyWithWings()
    }

    display() {
        console.log('Displays a rubber duck')
    }
}

function main() {
    for (const duck of [new RubberDuck(), new MallardDuck()]) {
        duck.display()
        duck.fly()
        duck.quack()
        duck.swim()
    }
}

main()
