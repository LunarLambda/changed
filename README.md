`Cd`: A wrapper type that tracks mutable borrows of the data it owns.

## Usage
```
use changed::Cd;

// Create the change tracker with an i32
let mut test: Changed<i32> = Changed::new(20);

// Mutate it (calling deref_mut through the *)
*test += 5;

// changed() reports whether or not it was changed
assert!(test.changed());

// Reset it the tracker back to false
test.reset();

// Read the data
assert_eq!(*test, 25);

// That didn't trip the change detection!
assert!(!test.changed());
```

## How it works
`Cd` tracks calls to `deref_mut()`, meaning it is possible to call `deref_mut()` and not change
it, creating a false positive. Use equality to detect actual changes to the value.

Along with that, there is a function to mutate a `Cd` without tripping change detection.
