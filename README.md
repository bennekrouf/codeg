

In the root of the folder, you put a orders.yml file with the content like :

endpoints:
- path: get_a_list_of_last_orders
  description: Retrieve a list of last orders
- path: create_new_order
  description: Create a new order
- path: update_order
  description: Update an existing order
- path: delete_order
  description: Delete an order
fields:
- name: CardCode
  properties:
  - mandatory
  - number


You run 

cargo run

It generates a folder generated with subfolders endpoints and proto



Then you run 

cargo build


It compiles the generated code
