const fs = require("fs");

let input = __dirname + "/input.txt";
let data : string[] = fs.readFileSync(input, 'utf8').split('\r\n')

let total : number = 0;
let totalsArray : number[] = [];

for (let food of data) {
    if (food != '') {
        total += parseInt(food);
    } else {
        totalsArray.push(total)
        total = 0;
    }
}

console.log("First part answer: " + totalsArray.sort((a,b) => b-a )[0])

let ans = totalsArray
            .sort((a,b) => b-a )
            .slice(0,3)
            .reduce((a,b) => a+b )

console.log("Second part answer: " + ans)

export {}