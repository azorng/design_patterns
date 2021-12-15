class Observer {
    static _observers = []
    _observables: any

    constructor() {
        this._observables = {}
        Observer._observers.push(this)
    }

    observe(kind: EvtKind, fn: any) {
        this._observables[kind] = fn
    }
}

enum EvtKind {
    GUEST_ARRIVED
}

class Evt {
    constructor(kind: EvtKind, data: any) {
        Observer._observers.forEach((observer) => {
            if (observer._observables[kind]) {
                observer._observables[kind](data)
            }
        })
    }
}

class Guest {
    name: string

    constructor(name: string) {
        this.name = name
    }
}

class Room extends Observer {
    guests: Guest[] = []

    constructor() {
        super()
        this.observe(EvtKind.GUEST_ARRIVED, this.on_guest_arrived.bind(this))
    }

    on_guest_arrived(guest: Guest) {
        this.guests.push(guest)
    }
}

const room = new Room()

new Evt(EvtKind.GUEST_ARRIVED, new Guest('Bob'))
new Evt(EvtKind.GUEST_ARRIVED, new Guest('Alice'))

console.log(room.guests)
