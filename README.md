# Rust REST API with warp

This code repository shows an example of how build a simple REST API using the Rust programming language and the warp crate.

It shows a create, read, update, and delete example for a single "resource": a shopping list item. 

There is an article on my blog explaining the details building this example: [How to implement a Rust REST API with warp](https://tms-dev-blog.com/how-to-implement-a-rust-rest-api-with-warp/).

## Install & Run

Simply run the project using cargo:

```bash
cargo run
```

## Using endpoints using curl

Here are some example commands to use when the REST API server is running:

Create an item:

```bash
curl -X POST 'http://localhost:5000/shopping_list_item' -H "Content-Type: application/json" -d '{"name": "peanut butter", "item_type": "spread", "description": "made from peanuts", "price": 2.5}'
```

Read list of items:

```bash
curl -o - 'http://localhost:5000/shopping_list_items' -H 'Content-Type application/json'
```

Update an item:

```bash
curl -X PUT 'localhost:5000/shopping_list_item/0' -H "Content-Type: application/json" -d '{"description": "made from real peanuts", "price": 4.5}'
```

Delete an item:

```bash
curl -X DELETE 'localhost:5000/shopping_list_item/0'
```