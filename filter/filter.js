// To run locally, use: node filter.js

const arr = [{ prop: 1, name: 'first' }, { prop: 2, name: 'second' }, { prop: 2, name: 'third' }];
const comparator = 2;

// filter `arr` down to include only the elements that have
// object `prop` properties of the `comparator` variable (2). 
// save the result into a new array called `res`.
const res = arr.filter(r => r.prop === comparator);

console.log(res); // [ { prop: 2, name: 'second' }, { prop: 2, name: 'third' } ]
