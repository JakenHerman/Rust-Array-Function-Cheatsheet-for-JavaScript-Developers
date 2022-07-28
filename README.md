# Rust Array Functions: A Cheatsheet for JavaScript Developers
A cheatsheet on how to use common JavaScript instance methods in Rust.

# Reduce

**JavaScript** documentation:

> Executes a user-supplied "reducer" callback function on each element of the array (from left to right), to reduce it to a single value.
> 

JavaScript Example:

```jsx
const res = arr.reduce((acc, obj) =>
  Math.abs(obj.prop) < Math.abs(acc.prop) ? obj : acc
);
```

**Rust** documentation:

> Reduces the elements to a single one, by repeatedly applying a reducing operation.
> 

Rust Example:

```rust
let res = arr.iter().reduce(|acc, obj| {
    if (obj.prop).abs() < (acc.prop).abs() {
        obj
    } else {
        acc
    }
});
```

---

# Filter

**JavaScript** documentation:

> Returns a new array containing all elements of the calling array for which the provided filtering function returns `true`.
> 

Example:

```jsx
const res = arr.filter(r => r.prop === comparator);
```

**Rust** documentation:

> Creates an iterator which uses a closure to determine if an element should be yielded.
> 

Example:

```rust
let res = arr.iter().filter(|&r| r.prop == comparator);
```

---

# Map

**JavaScript** documetation:

> Returns a new array containing the results of invoking a function on every element in the calling array.
> 

Example:

```jsx
let res = arr.map(r => ({
    name: r.displayName,
    family: r.familyName
});
```

**Rust** documentation:

> Takes a closure and creates an iterator which calls that closure on each element.
> 

Example:

```rust
let res = arr.map(|r| {
    name: r.display_name.clone(),
    family: r.family_name.clone(),
});
```
