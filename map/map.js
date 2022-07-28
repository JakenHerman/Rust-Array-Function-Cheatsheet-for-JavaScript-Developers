// To run locally, use: node map.js

// this is a silly example, but it's a good example of how to use the map function
const arr = [
  { displayName: 'John', familyName: 'Doe' },
  { displayName: 'Jane', familyName: 'Doe' },
  { displayName: 'Jaken', familyName: 'Herman' }
];

let res = arr.map(r => ({
  name: r.displayName,
  family: r.familyName
}));

console.log(res); 

/** ^ ^ ^ ^ ^ ^   
[
  { name: 'John', family: 'Doe' },
  { name: 'Jane', family: 'Doe' },
  { name: 'Jaken', family: 'Herman' }
]  
*/
