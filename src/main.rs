#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::response::content::Html;

#[get("/")]
fn index() -> Html<&'static str> {
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


fn main() {
    rocket::ignite()
        .mount("/", routes![index]) // Mounting the handlers
        .launch();
}
