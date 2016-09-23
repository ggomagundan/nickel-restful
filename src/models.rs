//extern crate rustc_serialize;

//use rustc_serialize;
//use time::Timespec;

#[derive(RustcEncodable , RustcDecodable, Debug )]
pub struct User{
  pub id: i32,
  pub username: Option<String>,
  pub email: Option<String>
}


#[derive(RustcEncodable , RustcDecodable)]
pub struct Car{
  pub id: i32,
  pub user_id: i32,
  pub car_name: Option<String>,
  pub car_number: String
}

