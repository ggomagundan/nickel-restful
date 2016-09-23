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
    let port: u16 = 4321;
/*
    let postgres_middleware: PostgresMiddleware = PostgresMiddleware::new(
        "postgres://postgres:postgres@localhost", postgres::NoSsl, 5);
*/
    let pool = my::Pool::new("mysql://root:@localhost:3306/awash_dev").unwrap();
    //server.utilize(Nickel::json_body_parser());
    //TODO cannot use static files middleware because of middleware chaining bug
    //See: https://github.com/nickel-org/nickel.rs/issues/59
    server.utilize(StaticFilesHandler::new("client/dist/"));


  server.utilize(middleware! { |request|
    println!("logging request from middleware! macro: {:?}", request.origin.uri);
  });



    //Routing
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
 //   return response.send(data_result);


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

     //let user_data = json::encode(&users).unwrap();

    //let mut data_result = "{\"status\": 200, \"msg\": \"OK\", \"data\": ".to_owned();
    //data_result.push_str(&user_data.to_string());

    //data_result.push_str("}");

    response.set(StatusCode::Ok);
    response.set(MediaType::Json);
//println!("{:?}",&users);
for i in &users {
println!("{:?}", &i.username);

}


let mut data = HashMap::new();
data.insert("dd", users);
/*
for info in &users {
data.insert("id", &info.username);
data.insert("username", &info.username);

//data.insert("username", &info.username.unwrap() );
//data.insert("email", &info.email);

}
*/



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
 //   return response.send(data_result);


    format!("{}", data_result)


});


    server.put("/api/persons/:id", middleware! { |request, mut response|




let form_data = try_with!(response, request.form_body());

  println!("{:?}", form_data);
  let mut data = HashMap::new();
  data.insert("username", form_data.get("lastname").unwrap_or("Last name?"));
  data.insert("email", form_data.get("email").unwrap_or("Email?"));
let id = 1;

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
 //   return response.send(data_result);


    format!("{}", data_result)



//println!("params {:?}", request.param.unwrap());

});


// get_persons);
    //server.post("/api/person", post_person);
    //server.put("/api/person", put_person);
    //server.delete("/api/person/:id", delete_person);

    println!("Listening on port {}", port);
    server.listen("0.0.0.0:6767");
}

/*
fn get_sole_iterable<T, I: Iterator<T>>(iter: &mut I) -> (Option<T>, i32) {
    //NOTE thanks, @sfackler
    match (iter.next(), iter.next()) {
        (None, _) => (None, 0),
        (Some(result), None) => (Some(result), 1),
        _ => (None, (iter.count() + 2))
    }
}
*/

/*
#[test]
fn test_get_sole_iterable() {
    let input: Vec<int> = vec![1 as int];
    let (val, count) = get_sole_iterable(&mut input.iter());
    assert_eq!(count, 1);
    assert_eq!(*(val.unwrap()), 1);

    let input: Vec<int> = vec![1 as int, 2 as int];
    let (val, count) = get_sole_iterable(&mut input.iter());
    assert_eq!(count, 2);
    match val {
        None => { assert!(true); },
        Some(_) => { assert!(false); }
    }

    let input: Vec<int> = vec![];
    let (val, count) = get_sole_iterable(&mut input.iter());
    assert_eq!(count, 0);
    match val {
        None => { assert!(true); },
        Some(_) => { assert!(false); }
    }
}
*/

fn set_response_content_type_json(response: &mut Response) {
response.set(StatusCode::Ok);
response.set(MediaType::Json);
/*
    response.origin.headers.content_type =
        Some(response.origin.headers.content_type.clone().unwrap_or(
            http::headers::content_type::MediaType {
                type_: String::from_str("application"),
                subtype: String::from_str("json"),
                parameters: Vec::new()
            })
        );
*/
}

//TODO find way to create a mock Response object for testing
//#[test]
// fn test_set_response_content_type_json() {
//     let mut resp = Response::new();
//     set_response_content_type_json(&mut resp);
// }

/*
fn get_comma_seperated_ids(input: &str) -> Vec<i32> {
    let strs = input.as_slice().split_str(",");
    let ids: Vec<i32> = strs.filter_map(from_str::<i32>).collect();
    ids
}

#[test]
fn test_get_comma_separated_ids() {
    assert_eq!(get_comma_seperated_ids("123,4,56"), vec![123,4,56]);
    assert_eq!(get_comma_seperated_ids("1111"), vec![1111]);
    //negative numbers are allowed
    assert_eq!(get_comma_seperated_ids("123,-5,56"), vec![123,-5,56]);
    //non-numbers are not allowed
    assert_eq!(get_comma_seperated_ids("123,f,56"), vec![123,56]);
    //whitespace is not allowed
    assert_eq!(get_comma_seperated_ids("123, 4,56"), vec![123,56]);
}
*/

fn get_persons (req: & Request, response: &mut Response) {

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
 //   return response.send(data_result);


   // format!("{}", data_result)


/*
    set_response_content_type_json(response);
    //TODO find a less verbose way to extract an i32 parameter from request
    let idsStr: &String = (req.params.get(&"ids".to_string()));
    let ids = get_comma_seperated_ids(idsStr.as_slice());
    println!("get_persons 1 ids={}", ids);

    let db_conn = req.db_conn();
    let stmt = db_conn.prepare(
        "SELECT id, name, created FROM person
        WHERE id = ANY( $1 )").unwrap();
    let idOptions: Vec<Option<i32>> = ids.iter().map( |i| Some(*i) ).collect();
    let idsForDb: postgres::types::array::ArrayBase<Option<i32>> =
        postgres::types::array::ArrayBase::from_vec(idOptions, 0);
    let mut iter = stmt.query(
        [&idsForDb]).unwrap();
    let mut persons: Vec<User> = Vec::new();
    for select in iter {
        let person = User {
            id: select.get(0),
            name: select.get(1),
            created: select.get(2)
        };
        persons.push(person);
    }
    let num_persons = persons.len();
    if num_persons == 0 {
        response.origin.status = http::status::NotFound;
        let result = ServerMessage {
            message: format!("{} persons were selected", num_persons)
        };
        let text = json::encode(&result);
        response.send(text.as_slice());
    }
    else {
        response.origin.status = http::status::Ok;
        let result = GetUsersMessage {
            persons: persons
        };
        let text = json::encode(&result);
        response.send(text.as_slice());
    }
*/
}

/*
fn get_person (req: &Request, response: &mut Response) {
    set_response_content_type_json(response);
    //TODO find a less verbose way to extract an i32 parameter from request
    let idStr: Option<int> = from_str(req.params.get(&"id".to_string()).as_slice());
    let id: i32 = idStr.unwrap() as i32;
    println!("get_person id={}", id);
    let db_conn = req.db_conn();
    let stmt = db_conn.prepare(
        "SELECT id, name, created FROM person WHERE id = $1").unwrap();
    let mut iter = stmt.query(
        [&id]).unwrap();

    let (maybeResult, selects) =
        get_sole_iterable(&mut iter);
    if selects == 1 {
        let select = maybeResult.unwrap();
        let result = User {
            id: select.get(0),
            name: select.get(1),
            created: select.get(2),
        };
        let text = json::encode(&result);
        response.send(text.as_slice());
    }
    else {
        let result = ServerMessage {
            message: format!("{} persons were selected", selects)
        };
        let text = json::encode(&result);
        if selects == 0 {
            response.origin.status = http::status::NotFound;
        }
        else if selects > 1 {
            response.origin.status = http::status::InternalServerError;
        }
        response.send(text.as_slice());
    }
}
*/

/*
fn post_person(req: &Request, response: &mut Response) {
    set_response_content_type_json(response);
    println!("post_person called");
    let person: UserByPost = req.json_as::<UserByPost>().unwrap();
    let db_conn = req.db_conn();
    let inserts = db_conn.execute(
        "INSERT INTO person (name, created) VALUES ( $1, $2 )",
        [&person.name.as_slice(), &time::get_time()]).unwrap();
    if inserts == 0 {
        response.origin.status = http::status::NotFound;
    }
    else if inserts > 1 {
        response.origin.status = http::status::InternalServerError;
    }
    let result = ServerMessage {
        message: format!("{} persons were inserted", inserts)
    };
    let text = json::encode(&result);
    response.send(text.as_slice());
    //TODO error checking to ensure that JSON decode succeeded
}

fn put_person(req: &Request, response: &mut Response) {
    set_response_content_type_json(response);
    println!("put_person called");
    let person: UserByPut = req.json_as::<UserByPut>().unwrap();
    let db_conn = req.db_conn();
    let updates = db_conn.execute(
        "UPDATE person SET ( name ) = ( $2 ) WHERE id = $1",
        [&person.id, &person.name.as_slice()]).unwrap();
    if updates == 0 {
        response.origin.status = http::status::NotFound;
    }
    else if updates > 1 {
        response.origin.status = http::status::InternalServerError;
    }
    let result = ServerMessage {
        message: format!("{} persons were updated", updates)
    };
    let text = json::encode(&result);
    response.send(text.as_slice());
    //TODO error checking top ensure that JSON decode succeeded
}

fn delete_person (req: &Request, response: &mut Response) {
    set_response_content_type_json(response);
    let idStr: Option<int> = from_str(req.params.get(&"id".to_string()).as_slice());
    let id: i32 = idStr.unwrap() as i32;
    println!("delete_person id={}", id);
    let db_conn = req.db_conn();
    let deletes = db_conn.execute(
        "DELETE FROM person WHERE id = $1",
        [&id]).unwrap();
    if deletes == 0 {
        response.origin.status = http::status::NotFound;
    }
    else if deletes > 1 {
        response.origin.status = http::status::InternalServerError;
    }
    let result = ServerMessage {
        message: format!("{} persons were deleted", deletes)
    };
    let text = json::encode(&result);
    response.send(text.as_slice());
    //TODO error checking to ensure that JSON decode succeeded
}
*/
