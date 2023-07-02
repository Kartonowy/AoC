const fs = require("fs")

let input : string = __dirname + "/input.txt"
let data = fs.readFileSync(input,"utf8").split('\r\n')
let sum : number = 0;

for (let pair of data) {
    pair = pair.split(",")
    pair[0] = pair[0].split("-")
    pair[1] = pair[1].split("-")
    if ((parseInt(pair[0][0]) >= parseInt(pair[1][0]) && parseInt(pair[0][1]) <= parseInt(pair[1][1])) 
        || 
        (parseInt(pair[1][0]) >= parseInt(pair[0][0]) && parseInt(pair[1][1]) <= parseInt(pair[0][1]))
        ) {
        sum += 1;
    }
}

console.log("Answer to part one is " + sum)


sum = 0;

for (let pair of data) {
    pair = pair.split(",")
    pair[0] = pair[0].split("-")
    pair[1] = pair[1].split("-")
    if (parseInt(pair[0][1]) >= parseInt(pair[1][0]) || parseInt(pair[1][1]) <= parseInt(pair[0][0])) {
        sum += 1;
    }
}

console.log("Answer to part two is " + sum)