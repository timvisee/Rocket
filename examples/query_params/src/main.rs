#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[derive(FromForm)]
struct Person {
    name: String,
    age: Option<u8>
}

#[get("/hello?<person>")]
fn hello(person: Person) -> String {
    if let Some(age) = person.age {
        format!("Hello, {} year old named {}!", age, person.name)
    } else {
        format!("Hello {}!", person.name)
    }
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
