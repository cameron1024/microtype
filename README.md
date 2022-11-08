# microtype
Boilerplate-free microtypes

### What/Why/How?

Microtypes (also known as "newtypes") are thin wrappers around primitive types, that differentiate between 2 otherwise identical types, based on their usage/meaning.
Since they are distinct types, they can't be substituted for one another. This helps to reduce logic bugs, by catching incorrect use of data at compile time.

For example, consider the following:
```rust
fn handle_order(user_id: String, order_id: String) {
  // ...
} 

fn main() {
  let user_id = ...;
  let order_id = ...;
  handle_order(order_id, user_id);
}
```

In this small example, it's fairly easy to see that there's a bug here: `order_id` and `user_id` are in the wrong order. However, as projects grow it becomes harder to spot.
More generally, human beings aren't very good at detecting errors like this, so we should try to offload this work onto the compiler.

Let's take a look at that but with microtypes instead:
```rust
// microtype definitions
microtype! {
  String {
    UserId
  }

  String {
    OrderId
  }
}

// or use the shorthand for declaring multiple microtypes
// microtype! {
//   String {
//     UserId
//     OrderId
//   }
// }

fn handle_order(user_id: UserId, order_id: OrderId) {
  // ...
} 

fn main() {
  let user_id: UserId = ...;
  let order_id: OrderId = ...;
  handle_order(order_id, user_id);  // Error, mismatched types
}
```
Great! It doesn't compile. By introducing microtypes, we've moved this run-time error into a compile-time error.

For more details and examples, check out the [docs](https://docs.rs/microtype)

### Contributing

Any and all contributions are always welcome! Feel free to raise an issue/submit a PR, etc.


