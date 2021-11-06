class FlyBehavior:
    def fly(self): pass

class FlyWithWings(FlyBehavior):
    def fly(self):
        print("LOOK AT ME I'M FLYINNN")

class NoFly(FlyBehavior):
    def fly(self):
        print("Wish I could fly but nope")

class Duck:
    fly_behavior: FlyBehavior

    def display(self):
        raise NotImplementedError

    def fly(self):
        self.fly_behavior.fly()

    def swim(self):
        print("Swiming is not a problem")

class MallardDuck(Duck):
    def __init__(self):
        self.fly_behavior = FlyWithWings()

    def display(self):
        print("BEHOLD. THE MIGTH BALLARD DUCK!!")

class RubberDuck(Duck):
    def __init__(self):
        self.fly_behavior = NoFly()

    def display(self):
        print("RUBBER RUBBER IS HERE!!")


duck: Duck
for duck in [RubberDuck(), MallardDuck()]:
    duck.display()
    duck.fly()
    duck.swim()





