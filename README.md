Rocket
Rocket is a web framework for Rust (nightly) with a focus on ease-of-use,
expressibility, and speed. Here's an example of a complete Rocket application:

#![feature(proc_macro_hygiene)]

#[macro_use] extern crate rocket;

#[get("/<name>/<age>")]

fn hello(name: String, age: u8) -> String {

    format!("Hello, {} year old named {}!", age, name)
}

fn main() {

    rocket::ignite().mount("/hello", routes![hello]).launch();
    
}


Visiting localhost:8000/hello/shubham/24, for example, will trigger the hello route resulting in the string Hello,

24 year old named shubham! being sent to the browser. If an <age> string was passed in that can't be parsed as a u8,

the route won't get called, resulting in a 404 error.


 
In this repository i foused on :

				Templates(Tera) and its rendring
				
				Routing 
				
				Using Static files like HTML , CSS , Images

 
