# Collecting Data from an API

Data is essential when building applications. 

Let's collect some data that we can use to write an application.

Continue reading if you want to:
- Query an API.
- Handle some Http errors.
- Collect some data and save it as JSON.
- Write an application in Rust.

If you are looking for the complete code you can find it [here](github.com/davidmaceachern/playground-data-collection-rust).

## Requirements

Before going further let's define how we can address the requirements of the application to avoid doing more than necessary.

- **Finding data** we need to have some open data we can collect from an API somewhere. It needs to be open because like Music and Films we need to have permission to use it.
- **Collecting data** we can do this using a HTTP client, it's worth noting that a data structure that is transmitted via HTTP is serialized as a string.
- **Storing data** can be done using a filesystem. This can be done by converting the strings we collect to a suitable data structure and outputting it to a file. The format we choose will depend on what we want to do with the data later on.

## Finding Data 

Firstly we need an API we can query for some data if you do not already have one chosen, a good place to start is this [repository](https://github.com/public-apis/public-apis).

For this exercise, we are going to get some [cat facts](https://alexwohlbruck.github.io/cat-facts/docs/). Taking a look through the repository we can see a [repository licence](https://github.com/alexwohlbruck/cat-facts/blob/master/LICENSE), which looks permissive enough to use this data.

We can use a web browser to call a `GET` endpoint, pasting this endpoint `https://cat-fact.herokuapp.com/facts/random` into the address bar returns the following response.

``` Json
{
  "used": false,
  "source": "api",
  "type": "cat",
  "deleted": false,
  "_id": "591f98703b90f7150a19c151",
  "__v": 0,
  "text": "Cat families usually play best in even numbers. Cats and kittens should be aquired in pairs whenever possible.",
  "updatedAt": "2020-06-30T20:20:33.478Z",
  "createdAt": "2018-01-04T01:10:54.673Z",
  "status": {
    "verified": true,
    "sentCount": 1
  },
  "user": "5a9ac18c7478810ea6c06381"
}
```

## Writing an Application in Rust

### Setting up the project

We are going to write our application in Rust, if you haven't already you can install Rust using the instructions [here](https://www.rust-lang.org/learn/get-started).

Next, we want to check that the installation provided the `Cargo` package manager, we can do this by running: 

``` Bash
$ cargo --version
```
If that returned the version we have, we can then initialize a new project that uses the current folder as the name of the project by running:

``` Bash
$ cargo init --bin
```

One thing I like about Rust is the ecosystem, as functionalities that other languages have built-in can be provided through `Crates` until the Rust Language team adopt them.

I would recommend installing `cargo-edit` for adding packages the same way you might do in Javascript when running `$ npm install --save`.

``` Rust
$ cargo install cargo-edit
```

To address the problem our application will solve, we can use the following crates together:
- The HTTP client we can use to send the request for our data is very helpfully called [reqwest](https://crates.io/crates/reqwest)
- Filesystem interactions will be provided by a JSON file store called [jfs](https://crates.io/crates/jfs)
- To convert our strings to data structures and data structures to strings we can use [serde](https://crates.io/crates/serde)
  - For dealing with JSON data structures we can use [serde_json](https://crates.io/crates/serde_json)
- To avoid worrying about how we implement errors for now we can use [anyhow](https://crates.io/crates/anyhow)


``` Bash
$ cargo add reqwest serde serde_json jfs anyhow 
```

The output will look like:

``` Bash
      Adding reqwest v0.10.7 to dependencies
      Adding serde v1.0.115 to dependencies
      Adding serde_json v1.0.57 to dependencies
      Adding jfs v0.6.2 to dependencies
      Adding anyhow v1.0.32 to dependencies
```

Without specifying a version number for these libraries, we will want to check the versions we are telling Cargo to use because that will determine which version of the documentation we need to look at.

Inside the `Cargo.toml` we can see:

``` Toml
[dependencies]
reqwest =  "0.10",
serde = "1.0.115",
serde_json = "1.0.57"
jfs = "0.6.2"
anyhow = "1.0.32"
```

So next we can build the application to install the dependencies, we can do this by running:

``` Rust
$ cargo build
```
What I am going to do from here onwards however:

``` Rust
$ cargo run
```
Which will build and execute the code for us so we can have some feedback from theError: Server responded with: 404 Not Found output:
``` Bash 
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/data-collection-rust`
Hello, world!!
```
Normally if we had tests written then we could have them watch for new  file changes but that is out of scope for the article today.

### Calling the API

We will start by replicating calling the API in our application.

In a similar way to how the Web Browser was our client before, we must have a client that will interact with the API.

``` diff
 fn main() {
-    println!("Hello, world!");
+    let client = reqwest::blocking::Client::new();
 }
```
Running this code, we encounter an error.
``` Bash
error[E0433]: failed to resolve: could not find `blocking` in `reqwest`
 --> src/main.rs:2:27
  |
2 |     let client = reqwest::blocking::Client::new();
  |                           ^^^^^^^^ could not find `blocking` in `reqwest`
```

When using some crates we must specify the features that our application will use in the `Cargo.toml`.

``` diff 
 [dependencies]
-reqwest = "0.10"
+reqwest = { version = "0.10", features = ["blocking"] }
```
Ok so let's add another line to call out endpoint
``` diff
     let client = reqwest::blocking::Client::new();
+    let uri = "https://cat-fact.herokuapp.com/fact/random";
+    let response = client.get(uri).send()?;
```

Upon running our application again we see another error:

``` Bash
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements...
```

We're using the `?` operator here to handle calling a function that could throw an error. Let's do as the compiler suggests:

``` diff
-fn main() {
+fn main() -> Result {
```
Seems that isn't exactly what the compiler wants:
``` Bash
error[E0107]: wrong number of type arguments: expected 2, found 0
 --> src/main.rs:1:14
  |
1 | fn main() -> Result {
  |              ^^^^^^ expected 2 type arguments
```
The Result will only be the return type if our code is successful, if not then this function will return an error. This is where we can use `anyhow`:
``` diff
-fn main() {
+fn main() -> Result<(),  anyhow::Error> {
```
Ok, so we have another compiler error...
``` Bash
error[E0308]: mismatched types
 --> src/main.rs:1:14
  |
1 | fn main() -> Result<(), anyhow::Error> {
  |    ----      ^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
```
We need to add the following it seems, for any successful outcome that doesn't throw an error.
``` diff
     let response = client.get(uri).send()?;
+    Ok(())
 })
```

So we probably want to know what the response looks like. We can take a quick look using the following macro:

``` diff
+    dbg!(response);
     Ok(())
```
And it appears that I might have mistyped the url, as we receive a `not found` error.

``` Bash
[src/main.rs:5] response = Response {
    url: "https://cat-fact.herokuapp.com/fact/random",
    status: 404,
    headers: {
        "server": "Cowboy",
        "connection": "keep-alive",
        "x-powered-by": "Express",
        "access-control-allow-origin": "*",
        "content-security-policy": "default-src 'none'",
        "x-content-type-options": "nosniff",
        "content-type": "text/html; charset=utf-8",
        "content-length": "150",
        "set-cookie": "connect.sid=s%3A5IS9zYZqbamwJECS6C5JrdcDfIBJ8epX.Lbh4Zl5C21jdFOyih1RgS1%2FiZr2c8jxbEc1l1XiwTvo; Path=/; HttpOnly",
        "date": "Tue, 25 Aug 2020 17:46:27 GMT",
        "via": "1.1 vegur",
    },
}
```

I checked the API documentation and indeed I had mistyped the url. 

``` diff
-    let uri = "https://cat-fact.herokuapp.com/fact/random";
+    let uri = "https://cat-fact.herokuapp.com/facts/random";
```
After correction, we get the correct status code.

``` diff
    url: "https://cat-fact.herokuapp.com/facts/random",
    status: 200,
    headers: {
```

It would probably be a good idea to handle errors when we don't get a `200` response. Let's check the response value so we can add a condition.

``` diff
-    dbg!(response);
+    dbg!(response.status());
```
Ok.
``` Bash
[src/main.rs:5] response.status() = 200
```
Now let's add a conditional.

``` diff
-    dbg!(response.status());
+    if(response.status() == 200) {
+        println!("{}", response.status());
+    }
```
Cool.

``` Bash
200 OK
```
However we want it to throw an error right, seems `reqwest` might let us do this, let's force it to fail again by adding the typo back in.

``` diff
-    let uri = "https://cat-fact.herokuapp.com/facts/random";
+    let uri = "https://cat-fact.herokuapp.com/fact/random";
     let response = client.get(uri).send()?;
-    if(response.status() == 200) {
+    if response.status().is_client_error() || response.status().is_server_error() {
         println!("{}", response.status());
     } 
```
And let's have the application return the error it encounters to avoid running any other code. We can do this by using a macro bundled with anyhow.

``` diff
+use anyhow::anyhow;
+
 fn main() -> Result<(), anyhow::Error> {
     let client = reqwest::blocking::Client::new();
     let uri = "https://cat-fact.herokuapp.com/facts/random";
     let response = client.get(uri).send()?;
     if(response.status().is_client_error() || response.status().is_server_error()) {
-        println!("{}", response.status());
+        return Err(anyhow!("Server responded with: {}", response.status()));
     } 
     Ok(())
```
Seems the compiler is warning us about something:
``` Bash
warning: unnecessary parentheses around `if` condition
```
Remove the parentheses and now upon receiving a status code that isn't `200`
``` Bash
Error: Server responded with: 404 Not Found
```
Great. Next let's look at getting our facts out of the response.


### TODO Deserializing Data
### TODO Persisting the data locally
### TODO Determining the Types


### Missing Features

``` toml
[dependencies]
reqwest = { version = "0.10", features = ["blocking", "json"] }
serde = { version = "1.0.115", features = ["derive"] }
serde_json = "1.0.57"
jfs = "0.6.2"
anyhow = "1.0.32"
```


### Persisting the data locally

We want the data to be used after our application has finished running. This is useful if the application crashes, or we have another application that will use the data elsewhere.

- We are writing a new file for each dataset that we collect. 
- We restrict the number of calls we are writing a single record at a time, so our program is synchronous.
- We can use a loop to run our program for the prescribed number of times, and we pause the thread's execution so that we don't flood the API with requests. This can be based on whether the API we are calling has a throttling limit.

In the future we can increase the collection frequency then we might want to consider a different storage layer that considers scalability.

### Determining the types

The serde crate can handle parsing untyped json as long as it is valid. We can do this before assigning a type by using `value`.

For our use case we can use an unsigned integer which we know will never be a negative number.

``` Rust
let mut count = 0u32;
```

Rust is a strongly typed language so this means we can define the types that our application needs to know about. They will look like the following.

Let's define types anyway to see how this is done, notice that I have chosen to use "animal_type" instead since "type" is a Rust reserved [keyword](https://doc.rust-lang.org/reference/types.html).

``` Rust
struct CatFact {
    used: bool,
    source: String,
    animal_type: String,
    deleted: String,
    _id: String,
    _v: i32,
    text: String,
    updated_at: String,
    created_at: String,
    status: Status,
    user: String
}

struct Status {
    verified: bool,
    sentCount: i32
}
```

``` Rust
Response {
    url: "https://cat-fact.herokuapp.com/facts/random",
    status: 200,
    headers: {
        "server": "Cowboy",
        "connection": "keep-alive",
        "x-powered-by": "Express",
        "access-control-allow-origin": "*",
        "content-type": "application/json; charset=utf-8",
        "content-length": "305",
        "etag": "W/\"131-4MvaJDAXqtlkSUWatxfF1BCPTek\"",
        "set-cookie": "connect.sid=s%3Ap0LnvYyEQUN9plmg--3mnx7DNd1dyhI4.Ms1oCBB5sWMAMwCtgfye572bD%2FBfxlCRkRRoq8cLzEY; Path=/; HttpOnly",
        "date": "Tue, 18 Aug 2020 20:04:01 GMT",
        "via": "1.1 vegur",
    },
}
```

## Other Issues

### Reserved keywords

When attempting to define the types for a Json record, if the field name (also known as a key) happens to be a reserved keyword then the compiler handily points this out. Thanks Rust!

``` Rust
error: expected identifier, found keyword `type`
  --> src/main.rs:34:5
   |
34 |     type: String,
   |     ^^^^ expected identifier, found keyword
   |
help: you can escape reserved keywords to use them as identifiers
   |
34 |     r#type: String,
   |     ^^^^^^

error: aborting due to previous error

error: could not compile `playground-data-collection-rust`.
```

### Compiler Warnings

This type of warning doesn't stop the code from compiling, it's a bit like Javascripts ESLint telling us that our code isn't entirely idiomatic. 
``` Bash
warning: structure field `sentCount` should have a snake case name
  --> src/main.rs:49:5
   |
49 |     sentCount: i32
   |     ^^^^^^^^^ help: convert the identifier to snake case: `sent_count`

warning: 3 warnings emitted
```

This can be disabled whilst developing if the noise gets in the way with the following at the top of the file.

`#![allow(non_snake_case)]`

## Future work

Some things that we could do next:
- Add a test that will mock calling the API.
- Create a data factory for generating random test data.
- Use a storage layer such as Amazon S3 to enable scaling.

Thanks for reading, I hope this helped you, please reach out to me on Twitter if you have any questions!