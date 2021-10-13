# Readme

## Information

For paidy interview

Question: Simple Restaurant API description

Link: https://github.com/paidy/interview/blob/master/SimpleRestaurantApi.md

Role: Senior Software Engineer - Consumer Finance Engineering

Candidate: Ho Tsz Chun (Stern)


## Functions

*Functional*
* Add: create order (and random prepare time between 5-15 mins) by using table id & item id\
  http://127.0.0.1:3000/create?table_id=1&item_id=abc
* Delete: remove order by using order id or (table id & item id)\
  http://127.0.0.1:3000/delete?table_id=1&order_id=1634097976883281
* Select: get all (remaining) items by table id\
  http://127.0.0.1:3000/check/?table_id=1
* Select: get item by order id or (table id & item id)\
  http://127.0.0.1:3000/check/?table_id=1&item_id=5

*Technical*
* limit number of orders for 1 table
* Does not have to counted down in real time


## Build & Run Step

### Build

To-do

### Run

To-do

