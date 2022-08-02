// To run locally, use: node sort.js

let arr = [{ prop: 3, name: 'first' }, { prop: 2, name: 'second' }, { prop: 1, name: 'third' }];

const res = arr.sort((a, b) => a.prop - b.prop);

console.log(res);
/** ^ ^ ^ ^ ^
  [
    { prop: 1, name: 'third' },
    { prop: 2, name: 'second' },
    { prop: 3, name: 'first' }
  ]
 */