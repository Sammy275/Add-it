#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::response::content::Html;
use rocket::request::Form;


#[derive(FromForm)]     //Making a struct to parse the user post request
struct UserForm {
    number: i32
}


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
        <p>Please Go to the '/addit/{number}'</p>
        <p>You can input any number inside the uri</p>
    </body>
</html>"#)
}


#[get("/addit/<number>")]
fn addit(number: i32) -> String {
    let new_number = number + 100;          //100 is being added into the requested number
    format!("You entered {}, We added 100, new number is {}", number, new_number)
}

#[post("/addit", data="<form>")]
fn user_post(form: Form<UserForm>) -> String {
    let new_number = form.number + 100;
    format!("You entered {}, We added 100, new number is {}", form.number, new_number)
}

#[get("/form")]                         //Handler to display html document for user to make a post request
fn form() -> Html<&'static str> {
    Html(r#"<!DOCTYPE html>
<html>
    <head>
        <title>Post</title>
    </head>
    <body>
        <h1>Type a number that you want to add to 100</h1>
        <br>
        <form method="POST" action="/addit">
            <input type="number" placeholer="Any Number" name="number"><br><br>
            <input type="submit" value="Send">
        </form>
    </body>
</html>"#)
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index, addit, form, user_post]) // Mounting the handlers
        .launch();  //launching the server
}


//Written by: Saim Irfan IOT049609
// will be updated for the purpose of practise
// Pakistan Zindabad