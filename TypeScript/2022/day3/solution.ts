const fs = require("fs")

let input : string = __dirname + "/input.txt"
let data = fs.readFileSync(input,"utf8").split('\r\n')

let priority : string = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
let sum : number = 0;

for (let rucksuck of data) {
    rucksuck = [rucksuck.substring(0, rucksuck.length / 2), rucksuck.substring(rucksuck.length / 2, rucksuck.length)]
    for (let item of rucksuck[0]) {
        if (rucksuck[1].includes(item)) {
            sum += priority.indexOf(item) + 1;
            break
        }
    }
}
console.log("Answer to part one is " + sum)

sum = 0;

for (let i = 0; i < data.length; i+=3) {
    for (let item of data[i]) {
        if (data[i+1].includes(item) && data[i+2].includes(item)) {
            sum += priority.indexOf(item) + 1;
            break
        }
    }
}

console.log("Answer to part two is " + sum)

export {}