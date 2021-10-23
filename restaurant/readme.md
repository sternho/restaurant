# Readme

## Information

For paidy interview

Question: Simple Restaurant API description

Link: https://github.com/paidy/interview/blob/master/SimpleRestaurantApi.md

Role: Senior Software Engineer - Consumer Finance Engineering

Candidate: Ho Tsz Chun (Stern)


## Functions

*Functional*
* Add: create order (and random prepare time between 5-15 minutes) by using table id & item id\
  [POST] http://localhost:8000/order \
  JSON Body:
  ```
  {
    "table_id": "table1",
    "item_id": ["item1", "item2"]
  }
  ```
* Delete: remove order by using order id\
  [DELETE] http://localhost:8000/order/{order_id}
* Select: get order information by order_id\
  [GET] http://localhost:8000/order/{order_id}
* Select: get table information and related orders \
  [GET] http://localhost:8000/table/{table_id}?item_id={item_id}

*framework & library*
* rocket, web server for handle and receive http request.
* chrono, for handle datetime
* uuid, for generate order_id
* rand, for random cook time
* serde, for parse JSON

*Technical*
* Use redis hash and list function to store the data. Avoid install the extra program and easy to run the program. Hence, I didn't use redisearch or other database.
* Except save and delete functions, order_service and table_service write as purely functional programming to make test case can be written without mocking.


## Build & Run Step

### Build

* go to the project root
* run "rustup default nightly" command to download nightly library for web server.
* run "cargo build" command for compile the application.

### Run

* make sure you have redis in local (redis://127.0.0.1/)
* go to the project root
* run "cargo run" command.

### Test case
* go to the project root
* run "cargo test" command.
