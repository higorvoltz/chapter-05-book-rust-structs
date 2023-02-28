struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
  width2: i32,
  height2: i32,
}

#[derive(Debug)]
struct Circle {
  radius: f32,
  pi: f32
}

impl Circle {
  fn areaV4(&self) -> f32 {
    self.radius * self.pi
  }
}

fn main(){
  let mut user1 = User {
    active: true,
    username: String::from("newuser123"),
    email: String::from("newuser@gmail.com"),
    sign_in_count: 1,
  };

  println!("{}", user1.email); // accessing the email key with '.'

  user1.email = String::from("updateemail@gmail.com");

  println!("{}", user1.email);

  let user2name = String::from("user2name");
  let user2email = String::from("user2email@gmail.com");
  let new_user = build_user(user2name, user2email);

  let another_user = User {
    active: user1.active,
    username: user1.username,
    email: user1.email,
    sign_in_count: user1.sign_in_count
  };

  //Using Tuple Structs Without Named Fields to Create Different Types
  let color_rgb = Color(255,255,255);

  //calculate area of rectangles without structures
  let width1 = 10;
  let height1 = 5;
  println!("{}", areaV1(width1, height1));

  //refactoring with tuples
  let rect1 = (10,5);
  println!("{}", areaV2(rect1));

  //Refactoring with Structs: Adding More Meaning
  let rect2 = Rectangle {
    width2: 10,
    height2: 5,
  };

  println!("{}", areaV3(&rect2));

  println!();

  // Method Sytax
  let circle = Circle {
    radius: 1.0,
    pi: 3.14
  };

  println!("{}", circle.areaV4());


}

fn build_user (email: String, username: String) -> User {
  User {
    active: false,
    username,
    email,
    sign_in_count: 1,
  }
}

fn areaV1(width: u32, height: u32) -> u32 {
  width * height
}

fn areaV2(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

fn areaV3(rectangle: &Rectangle) -> i32 {
  rectangle.width2 * rectangle.height2
}

fn areaV4(circle: &Circle) -> f32 {
  circle.radius * circle.pi
}