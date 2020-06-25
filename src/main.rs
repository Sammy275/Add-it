#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::response::content::Html;

#[get("/")]
fn index() -> Html<&'static str> { //Returning an html page
    Html(r#"<!DOCTYPE html>
<html>
    <head>
        <title>Home</title>
    </head>
    <body>
        <h1>Welcome</h1>
        <p>This is a rocket practise</p>
        <p>Please Go to the '/add/{number}'</p>
        <p>You can input any number inside the uri</p>
    </body>
</html>"#)
}


#[get("/addit/<number>")]
fn addit(number: i32) -> String {
    let new_number = number + 100;          //100 is being added into the requested number
    format!("You entered {}, We added 100, new number is {}", number, new_number)
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index, addit]) // Mounting the handlers
        .launch();  //launching the server
}


//Written by: Saim Irfan IOT049609
// will be updated for the purpose of practise
// Pakistan Zindabad