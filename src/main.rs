//extern crate http;
extern crate mysql;
extern crate hyper;
extern crate time;
extern crate rustc_serialize;
#[macro_use] extern crate nickel;

//use std::io;
use std::fmt;
use std::fmt::Debug;
use std::str;
use nickel::{ Nickel, Request, Response, HttpRouter, JsonBody, MediaType, StaticFilesHandler, Middleware , FormBody};
use nickel::status::*; 

use rustc_serialize::json;

use mysql as my;
use std::collections::HashMap;

use models::{User, Car};


mod models;

fn main() {
    //Initialise server instance including all middleware
    let mut server = Nickel::new();
    let port: u16 = 6767;
    let pool = my::Pool::new("mysql://root:@localhost:3306/awash_dev").unwrap();

    server.utilize(StaticFilesHandler::new("client/dist/"));


  server.utilize(middleware! { |request|
    println!("logging request from middleware! macro: {:?}", request.origin.uri);
  });



    //Routing GET LIST
  server.get("/api/persons", middleware! { |request, mut response|



let pool = my::Pool::new("mysql://root:@localhost:3306/awash_dev").unwrap();
    let users: Vec<User> = pool.prep_exec("SELECT id, username, email FROM users limit 10", ())
      .map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
          let (id, username, email) = my::from_row(row);
            User {
              id: id,
              username: username,
              email: email
            }
          }).collect()
        }).unwrap();

    let user_data = json::encode(&users).unwrap();

    let mut data_result = "{\"status\": 200, \"msg\": \"OK\", \"data\": ".to_owned();
    data_result.push_str(&user_data.to_string());

    data_result.push_str("}");

    response.set(StatusCode::Ok);
    response.set(MediaType::Json);


    format!("{}", data_result)


});



    //Routing  POST LIST
    server.post("/api/persons", middleware! { |request, mut response|



 let form_data = try_with!(response, request.form_body());


 let user_name  = form_data.get("user_name").unwrap();
 let email = form_data.get("email").unwrap();

  let pool = my::Pool::new("mysql://root:@localhost:3306/awash_dev").unwrap();

println!("{}, {}",  user_name, email);
  let row  =  pool.prep_exec(format!("INSERT INTO users (username, email) VALUES('{}', '{}')",user_name, email), ()).unwrap();

  let id: i32 = row.last_insert_id() as i32;
println!("{}, {}, {}", id, user_name, email);
    let user =  
             User {
               id: id,
               username: Some(user_name.to_string()),
               email: Some(email.to_string())
             };

    let user_data = json::encode(&user).unwrap();

    let mut data_result = "{\"status\": 200, \"msg\": \"OK\", \"data\": ".to_owned();
    data_result.push_str(&user_data.to_string());

    data_result.push_str("}");

    response.set(StatusCode::Ok);
    response.set(MediaType::Json);


   format!("{}", data_result)


});


    server.get("/api/person_list/:id", middleware! { |request, mut response|


 let id = request.param("id").unwrap();
println!("list select {}",id);

let pool = my::Pool::new("mysql://root:@localhost:3306/awash_dev").unwrap();
    let users: Vec<User> = pool.prep_exec(format!("SELECT id, username, email FROM users where id = {}", id), ())
      .map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
          let (id, username, email) = my::from_row(row);
            User {
              id: id,
              username: username,
              email: email
            }
          }).collect()
        }).unwrap();

    response.set(StatusCode::Ok);
    response.set(MediaType::Json);
//println!("{:?}",&users);
for i in &users {
println!("{:?}", &i.username);

}


let mut data = HashMap::new();
data.insert("dd", users);

return response.render("views/show.json.tpl", &data)


});




    server.get("/api/persons/:id", middleware! { |request, mut response|


 let id = request.param("id").unwrap();
println!("select {}",id);

let pool = my::Pool::new("mysql://root:@localhost:3306/awash_dev").unwrap();
    let users: Vec<User> = pool.prep_exec(format!("SELECT id, username, email FROM users where id = {}", id), ())
      .map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
          let (id, username, email) = my::from_row(row);
            User {
              id: id,
              username: username,
              email: email
            }
          }).collect()
        }).unwrap();

    let user_data = json::encode(&users).unwrap();

    let mut data_result = "{\"status\": 200, \"msg\": \"OK\", \"data\": ".to_owned();
    data_result.push_str(&user_data.to_string());

    data_result.push_str("}");

    response.set(StatusCode::Ok);
    response.set(MediaType::Json);

    format!("{}", data_result)


});


    //Routing GET 
    server.put("/api/persons/:id", middleware! { | request, mut response|



 let form_data = try_with!(response, request.form_body());


 let username  = form_data.get("username").unwrap();
 let email = form_data.get("email").unwrap();

let id = 1;


  let pool = my::Pool::new("mysql://root:@localhost:3306/awash_dev").unwrap();

println!("{}, {}",  username, email);


    let row = pool.prep_exec(format!("update users set username = '{}', email = '{}' where id = {}",username, email,  id), ()).unwrap();


  println!("{}, {}, {}", id, username, email);
  let user =
    User {
      id: id ,
      username: Some(username.to_string()),
      email: Some(email.to_string())
    };


    let user_data = json::encode(&user).unwrap();

    let mut data_result = "{\"status\": 200, \"msg\": \"OK\", \"data\": ".to_owned();
    data_result.push_str(&user_data.to_string());

    data_result.push_str("}");

    response.set(StatusCode::Ok);
    response.set(MediaType::Json);


    format!("{}", data_result)




});


    //server.post("/api/person", post_person);
    //server.put("/api/person", put_person);
    //server.delete("/api/person/:id", delete_person);

    println!("Listening on port {}", port);
    server.listen("0.0.0.0:6767");
}

fn set_response_content_type_json(response: &mut Response) {
  response.set(StatusCode::Ok);
  response.set(MediaType::Json);
}


