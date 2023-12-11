const fs = require("fs")

let input : string = __dirname + "/strategyguide.txt"
let data = fs.readFileSync(input,"utf8").replaceAll(' ', '').replaceAll('A','X').replaceAll('B','Y').replaceAll('C','Z').split('\r\n')

let total : number = 0;
for (let match of data) {
    total += check(match);
}
console.log("Answer to part one is: " + total)

total = 0;
for (let match of data) {
    total += check2(match);
}

console.log("Answer for part two is " + total)


function check(shapes : string) {
    let score : number = 0;
    switch (shapes[1]) {
        case 'X': // Rock
            score += 1;
            switch (shapes[0]) {
                case 'X':
                    score += 3;
                    break
                case 'Y':
                    score += 0;
                    break
                case 'Z':
                    score += 6;
                    break
                default:
                    throw new Error();
            }
            break
        case 'Y': // Paper
            score += 2;
            switch (shapes[0]) {
                case 'X':
                    score += 6;
                    break
                case 'Y':
                    score += 3;
                    break
                case 'Z':
                    score += 0;
                    break
                default:
                    throw new Error();
            }
            break
        case 'Z': // Scissors
            score += 3;
            switch (shapes[0]) {
                case 'X':
                    score += 0;
                    break
                case 'Y':
                    score += 6;
                    break
                case 'Z':
                    score += 3;
                    break
                default:
                    throw new Error();
            }
            break
        default:
            throw new Error();
    }
    return score
}

function check2(shapes : string) {
    let score : number = 0;
    switch (shapes[0]) {
        case 'X': // Rock
            switch (shapes[1]) {
                case 'X': // need to lose
                    score += 0; // for losing
                    score += 3; // for scissors
                    break
                case 'Y': // need to tie
                    score += 3; // for losing
                    score += 1; // for rock
                    break
                case 'Z': // need to win
                    score += 6; // for losing
                    score += 2; // for paper
                    break
                default:
                    throw new Error()
            }
            break
        case 'Y': // Paper
            switch (shapes[1]) {
                case 'X': // need to lose
                    score += 0; // for losing
                    score += 1; // for rock
                    break
                case 'Y': // need to tie
                    score += 3; // for losing
                    score += 2; // for paper
                    break
                case 'Z': // need to win
                    score += 6; // for losing
                    score += 3; // for scissors
                    break
                default:
                    throw new Error()
            }
            break
        case 'Z': // Scissors
            switch (shapes[1]) {
                case 'X': // need to lose
                    score += 0; // for losing
                    score += 2; // for scissors
                    break
                case 'Y': // need to tie
                    score += 3; // for losing
                    score += 3; // for rock
                    break
                case 'Z': // need to win
                    score += 6; // for losing
                    score += 1; // for paper
                    break
                default:
                    throw new Error()
            }
            break
        default:
            throw new Error();
    }
    return score
}

export {}