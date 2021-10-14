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
  http://127.0.0.1:3000/create?table_id={}&item_id={}
* Delete: remove order by using order id or (table id & item id)\
  http://127.0.0.1:3000/delete?table_id={}&order_id={}
* Select: get all (remaining) items by table id\
  http://127.0.0.1:3000/check/?table_id={}
* Select: get item by order id or (table id & item id)\
  http://127.0.0.1:3000/check/?table_id={}}&item_id={}

*Technical*
* limit number of orders for 1 table
* Does not have to counted down in real time


## Build & Run Step

### Build

To-do

### Run

To-do

