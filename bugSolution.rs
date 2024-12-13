Several solutions exist depending on the desired behavior:

**1. Cloning:** If the data is cheap to copy, cloning the value is a simple and often effective solution:

```rust
fn main() {
    let mut x = 5;
    let y = x.clone(); // clone x to avoid borrow conflict
    let z = x.clone();// clone x again to avoid borrow conflict
    
    *y = 10; 
    *z = 100;
    println!("y = {}, z = {}, x = {}",y, z, x);
}
```

**2. Using RefCell:** For situations where cloning is not practical, `RefCell` provides interior mutability:

```rust
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);
    let mut y = x.borrow_mut();
    let mut z = x.borrow_mut(); //This will panic at runtime
    *y = 10;
    *z = 100;
}
```

**3. Restructuring the code:** Often, the best solution is to refactor your code to avoid needing multiple mutable borrows. This may involve changing data structures or algorithms.  This is highly context-dependent and can't be shown without a specific use-case scenario.

**Choosing the right solution:** The best approach depends on your specific use-case.  Cloning is easiest for small data. `RefCell` handles mutable references in situations where cloning is not suitable. However, keep in mind that `RefCell`'s runtime checks might introduce performance overhead and runtime panics.  Restructuring is usually the most preferred and idiomatic option where feasible, as it prevents runtime checks and reduces chances of errors.