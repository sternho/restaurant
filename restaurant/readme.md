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
* Delete: remove order by using order id or (table id & item id)\
  [DELETE] http://localhost:8000/order?order_id={}&table_id={}
* Select: get all items by table id (would hidden after order cooked finish)\
  [GET] http://localhost:8000/order \
  JSON Body:
  ```
  {
      "table_id": "table1",
      "item_id": "item2"
  }
  ```

*Technical*
* Add order_service to handle function logic as purely functional programming
* Redis


## Build & Run Step

### Build

* go to the project root
* run "rustup default nightly" command to download nightly library for web server.
* run "cargo build" command for compile the application.

### Run

* make sure you have redis in local (redis://127.0.0.1/)
* go to the project root
* run "cargo run" command.

