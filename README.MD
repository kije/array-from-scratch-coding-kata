# Array from scratch Kata
This is an implementation of the "array from scratch" coding kata.

## Kata
Implement an Array/List-Like Data-Structure from scratch

Write an array/list like data structure, without using any pre-existing container/collection data structures like arrays, objects, vectors, sets, maps, lists, dictionaries, etc... to store the items

- Your implementation should support storing an arbitrary number of items in (1 or more)
- It should support random access (i.e. I can retrieve the n-th element of the list/array) and appending new values to the end of the list
- It should at least support storing numbers (integers), but optionally support more complex data types as items (however, this is no required to be a valid solution for this kata)

## Implementation
This is an implementation in Rust. The array is implemented as a chain of nested closures, each capturing the element stored as it's item and an optional rest/remainder of the list.
The closure itself accepts a parameter to indicate if the value or the remainder of the list shall be returned.

Thus conceptually, this structure acts like a nested set of 2-element tuples
```
Closure(value3, Closure(value2, Closure(value1, None)))
```

Note: the array has a LIFO structure, as appending a new item to the list creates a new closure with the previous array-closure as it's remainder.

### Characteristics
- The closures are heap allocated and reference counted to avoid lifetime and unknown size at compile-time problems.
- Accessing the first item of a list is a more expensive operation than accessing the last element of the list (as items are stored in LIFO order).
- Large arrays may cause a stack overflow.