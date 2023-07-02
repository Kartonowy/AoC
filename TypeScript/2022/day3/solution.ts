const fs = require("fs")

let input : string = __dirname + "/input.txt"
let data = fs.readFileSync(input,"utf8").split('\r\n')

console.log(data)