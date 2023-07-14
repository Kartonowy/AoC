const fs = require("fs");

let input: string = __dirname + "/input.txt";
let data: string = fs.readFileSync(input, "utf8");

let alpha: string, omega: string;

let current: string[] = [];

function partOne(data: string) {
  for (let i = 0; i < data.length; i++) {
    if (current.length > 4) {
      current.shift();
    }
    if (!current.some((val, i) => current.indexOf(val) !== i) && i > 4) {
      return i;
    }
    current.push(data[i]);
  }
}

function partTwo(data: string) {
  for (let i = 0; i < data.length; i++) {
    if (current.length > 14) {
      current.shift();
    }
    if (!current.some((val, i) => current.indexOf(val) !== i) && i > 4) {
      return i;
    }
    current.push(data[i]);
  }
}

console.log("Answer to part one is " + partOne(data));
console.log("Answer to part two is " + partTwo(data));
