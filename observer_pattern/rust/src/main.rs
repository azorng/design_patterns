trait Subject {}

trait Publisher<T: Subject> {
    fn subscribe(&mut self, subscriber: Box<dyn Subscriber<T>>);
    fn notify_subscribers(&mut self);
}

trait Subscriber<T: Subject> {
    fn update(&self, subjects: &Vec<T>);
}

struct Person {
    name: String,
    interest: MovieGenre,
}
impl Person {
    fn new(name: String, interest: MovieGenre) -> Self {
        Person { name, interest }
    }
}
impl Subscriber<Movie> for Person {
    fn update(&self, movies: &Vec<Movie>) {
        for movie in movies {
            if movie.genre == self.interest {
                let msg = match movie.genre {
                    MovieGenre::Horror => "So scary",
                    MovieGenre::Action => "Finally some action",
                };
                println!("{}: Great!! {} was just released!! {}!!", self.name, movie.subject, msg);
            }
        }
    }
}

struct Movie {
    subject: String,
    genre: MovieGenre,
}
impl Subject for Movie {}
impl Movie {
    fn new(subject: String, genre: MovieGenre) -> Self {
        Movie { subject, genre }
    }
}
#[derive(PartialEq)]
enum MovieGenre {
    Horror,
    Action,
}

struct Cinema {
    movie_subscribers: Vec<Box<dyn Subscriber<Movie>>>,
    new_movies: Vec<Movie>,
}
impl Cinema {
    fn new() -> Self {
        Cinema {
            movie_subscribers: vec![],
            new_movies: vec![],
        }
    }

    fn add_new_movie(&mut self, movie: Movie) {
        self.new_movies.push(movie);
        self.notify_subscribers();
    }
}
impl Publisher<Movie> for Cinema {
    fn subscribe(&mut self, subscriber: Box<dyn Subscriber<Movie>>) {
        self.movie_subscribers.push(subscriber);
    }

    fn notify_subscribers(&mut self) {
        for s in &self.movie_subscribers {
            s.update(&self.new_movies);
        }
        self.new_movies = vec![];
    }
}

fn main() {
    let mut cinema = Cinema::new();

    let person_a = Person::new("Azor".to_string(), MovieGenre::Horror);
    let person_b = Person::new("Pepe".to_string(), MovieGenre::Action);

    cinema.subscribe(Box::new(person_a));
    cinema.subscribe(Box::new(person_b));

    let new_movies = vec![
        Movie::new("Spiderman".to_string(), MovieGenre::Action),
        Movie::new("Slenderman".to_string(), MovieGenre::Horror),
    ];

    for movie in new_movies {
        cinema.add_new_movie(movie);
    }
}
