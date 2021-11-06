interface Flyable {
  fly(): void;
}

interface Quackable {
  quack(): void;
}

class FlyWithWings implements Flyable {
  fly() {
    console.log("I am flying with wings");
  }
}

class NoFly implements Flyable {
  fly() {
    console.log("I can not fly :(");
  }
}

class Quack implements Quackable {
  quack() {
    console.log("QUAAACK");
  }
}

class NoQuack implements Quackable {
  quack() {
    console.log("I can not quack :(");
  }
}

abstract class Duck {
  protected flyBehavior?: Flyable;
  protected quackBehavior?: Quackable;
  abstract display(): void;

  fly() {
    this.flyBehavior?.fly();
  }

  quack() {
    this.quackBehavior?.quack();
  }
}

class RubberDuck extends Duck {
  constructor() {
    super();
    this.quackBehavior = new Quack();
  }

  display() {
    console.log("Displays a rubber duck");
  }
}

const duck = new RubberDuck();
duck.fly();
duck.quack();
duck.display();
