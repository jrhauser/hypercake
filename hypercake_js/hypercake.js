//I acknowledge the academic integrity policy and all work is my own
// - J. R. Hauser

//let me get commandline input with prompt since it isn't in node.js
// run "npm install prompt-sync" on your command line
const prompt = require("prompt-sync")();

//intialize lookup
const lookup = {};

let n = prompt("How many cuts? ");
let k = prompt("How many dimensions? ");
if (k >= 2) {
  console.log(hypercake(n, k));
} else {
  console.log("Not enough dimensions");
}

function hypercake(n, k) {
  let pieces = 0;
  if (n == 0) {
    return 1;
  } else {
    for (let i = 0; i <= k; i++) {
      pieces += combinations(n, i);
    }
    return pieces;
  }
  function combinations(n, r) {
    if (r >= 0 && r <= n) {
      combination = factorial(n) / (factorial(r) * factorial(n - r));
      return combination;
    } else {
      return 0;
    }
    function factorial(n) {
      if (n <= 1) {
        return 1;
      } else {
        if (n in lookup) {
          return lookup[n];
        } else {
          value = n * factorial(n - 1);
          lookup[n] = value;
          return value;
        }
      }
    }
  }
}
