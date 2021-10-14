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
  [GET] http://127.0.0.1:3000/create?table_id={}&item_id={},{},{}
* Delete: remove order by using order id or (table id & item id)\
  [GET] http://127.0.0.1:3000/delete?table_id={}&order_id={}
* Select: get all items by table id (would hidden after order cooked finish)\
  [GET] http://127.0.0.1:3000/check/?table_id={}
* Select: get item by table id & item id (would hidden after order cooked finish)\
  [GET] http://127.0.0.1:3000/check/?table_id={}}&item_id={}
* Select: get all items by table id & order id (would not hidden after order cooked finish)\
  [GET] http://127.0.0.1:3000/check/?table_id={}

*Technical*
* Add order_service to handle function logic as purely functional programming
* Redis


## Build & Run Step

### Build

* go to the project root
* run "cargo build" command.

### Run

* make sure you have redis in local (redis://127.0.0.1/)
* go to the project root
* run "cargo run" command.

