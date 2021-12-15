class Observer():
    _observers = []

    def __init__(self):
        self._observers.append(self)
        self._observables = {}

    def observe(self, event_name, callback):
        self._observables[event_name] = callback


class Event():
    def __init__(self, name, data):
        self.name = name
        self.data = data

        for observer in Observer._observers:
            if self.name in observer._observables:
                observer._observables[self.name](self.data)

class Room(Observer):
    def __init__(self):
        Observer.__init__(self) # Observer's init needs to be called

    def someone_arrived(self, who):
        print(who + " has arrived!")

room = Room()
room.observe('someone arrived',  room.someone_arrived)

Event('someone arrived', 'Lenard')
Event('someone arrived', 'johnny')


