const fs = require("fs")

let input : string = __dirname + "/input.txt"
let data = fs.readFileSync(input,"utf8").split('\r\n')
let sum : number = 0;

for (let pair of data) {
    pair = pair.split(",")
    pair[0] = pair[0].split("-")
    pair[1] = pair[1].split("-")
    if ((Number(pair[0][0]) >= Number(pair[1][0]) && Number(pair[0][1]) <= Number(pair[1][1])) 
        || 
        (Number(pair[1][0]) >= Number(pair[0][0]) && Number(pair[1][1]) <= Number(pair[0][1]))
        ) {
        sum++;
    }
}

console.log("Answer to part one is " + sum)


sum = 0;

for (let pair of data) {
    pair = pair.split(",")
    pair[0] = pair[0].split("-")
    pair[1] = pair[1].split("-")
    if ((Number(pair[0][0]) <= Number(pair[1][0]) && Number(pair[0][1]) >= Number(pair[1][1]))
    || (Number(pair[0][0]) >= Number(pair[1][0]) && Number(pair[0][1]) <= Number(pair[1][1]))
    || (Number(pair[0][0]) <= Number(pair[1][1]) && Number(pair[1][0]) <= Number(pair[0][1]))
    ) {
        sum++;
    }
}

console.log("Answer to part two is " + sum)