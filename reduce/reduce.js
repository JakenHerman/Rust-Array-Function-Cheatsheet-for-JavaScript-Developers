// To run locally, use: node reduce.js

let arr = [{ prop: 1, name: 'first' }, { prop: 2, name: 'second' }, { prop: -2, name: 'third' }];

const res = arr.reduce((acc, obj) =>
  Math.abs(obj.prop) < Math.abs(acc.prop) ? obj : acc
);

console.log(res); // { prop: 1, name: 'first' }