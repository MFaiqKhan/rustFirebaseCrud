use firebase_rs::*; // this line interperates the firebase_rs crate, 
//* means all the functions in the crate are available to use in this file
use serde::{Serialize, Deserialize}; // serializing and deserializing data to and from JSON
use std::collections::HashMap; // this is used to store the data we get from the database

// using two structs to represent the data we want to send to the database

#[derive(Serialize, Deserialize, Debug)] // rust understand structs but the database doesn't, 
//so we need to serialize and deserialize with json the data 
struct User { // this struct is used to send the data to the database
    name: String,
    age: u32,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]  
struct Response { // this struct is used to get the response from the database
    name: String,
}

#[tokio::main] // this is a macro that tells the compiler to run the main function asynchronously
async fn main() {
    let user = User { // this is the data we want to send to the database, of type User struct
        name: "John Doe".to_string(), // to_string() converts the string literal to a String type
        age: 25, 
        email: "johndoe@gmail.com".to_string(), 
    };

    // Firebase::new() creates a new instance of the Firebase struct
    let firebase = Firebase::new("https://rust-crud-ef9ab-default-rtdb.firebaseio.com/").unwrap(); // this is the url of the database

    // set_user() is a function that takes a reference to the Firebase struct and a reference to the User struct
    // we are taking references because we don't want to move the data, we just want to borrow it
    let response = set_user(&firebase, &user).await; // this is the response we give to the database

    // get_users() is a function that takes a reference to the Firebase struct and a reference to the Response struct
    // thats for single user, if we want to get all the users we use get_users()
    let mut user = get_user(&firebase, &response.name).await; // this is the data we get from the database
    println!("{:?}", user); // this prints the data to the console

    // this gets all the users in the database
    let users = get_users(&firebase).await; // this is the data we get from the database
    println!("{:?}", users); // this prints the data to the console

    user.email = "newupdatedemail@gmail.com".to_string(); // we update the email of the user
    let updated_user = update_user(&firebase, &response.name, &user).await; // this is the theing we give to the database
    println!("{:?}", updated_user); // this prints the data to the console

    // await is used because the function is asynchronous
    delete_user(&firebase, &response.name).await; // this deletes the user from the database
    println!("User deleted"); // this prints to the console

}


async fn set_user(firebase_client: &Firebase, user: &User) -> Response {
    let firebase = firebase_client.at("users"); // this is the path to the users collection in the database in the database
    let _users = firebase.set::<User>(&user).await; // this sends the data to the database
    // we use the set() function from the Firebase struct to send the data to the database
    // <User> is used to tell the compiler that the data we are sending is of type User struct
    //( &user ) is used to tell the compiler that we are borrowing the data from the user variable
    return string_to_response(&_users.unwrap().data); // this returns the response from the database, we have unwrapped the data because it is wrapped in an Option and
    // that is because the data we get from the database is of type Option<String>
}

async fn get_users(firebase_client: &Firebase) -> HashMap<String, User> {
    let firebase = firebase_client.at("users"); // this is the path to the users collection in the database in the database
    let users = firebase.get::<HashMap<String, User>>().await; // this gets the data from the database
    println!("{:?}", users); // this prints the data to the console
    return users.unwrap(); // this returns the data from the database
}

async fn get_user(firebase_client: &Firebase, id: &String) -> User {
    let firebase = firebase_client.at("users").at(&id); // this is the path to the users collection in the database in the database
    let user =  firebase.get::<User>().await;// this gets the data from the database
    // why () after get::<User> ?  because we are not sending any data to the database
    println!("{:?}", user); // this prints the data to the console
    return user.unwrap(); // this returns the data from the database
}

async fn update_user(firebase_client: &Firebase, id: &String, user: &User) -> User {
    let firebase = firebase_client.at("users").at(&id); // this is the path to the users collection in the database in the database
    let _user = firebase.update::<User>(&user).await; // this updates the data(&user) in the database
    return string_to_user(&_user.unwrap().data); // this returns the data from the database
}

async fn delete_user( firebase_client: &Firebase, id: &String) {
    let firebase = firebase_client.at("users").at(&id); // this is the path to the users collection in the database in the database
    let _user = firebase.delete().await.unwrap(); // this deletes the data from the database
}


// Helper functions ( tp serialize and deserialize data to and from JSON)

fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap() // this converts the string to a Response struct, how ? 
    // we use the from_str() function from the serde_json crate to convert the string to a Response struct
}

fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap() // this converts the string to a User struct, how ? 
    // we use the from_str() function from the serde_json crate to convert the string to a User struct
}
