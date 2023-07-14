const fs = require("fs");

let input: string = __dirname + "/input.txt";
let data: string = fs.readFileSync(input, "utf8");

let stacks: string[][] | any[] = [[], [], [], [], [], [], [], [], []];

let stringStacks: string = data
  .split(/\r\n\r\n/)[0]
  .replaceAll("\r\n", "")
  .split("1")[0];
let instructions: string[] = data.split(/\r\n\r\n/)[1].split("\r\n");

let curStack = 1,
  endCheck = false;

let answer: string = "";

// parsing initial stacks
for (let i = 1; i < stringStacks.length; i += endCheck ? 3 : 4) {
  if (stringStacks[i] != " ") stacks[curStack - 1].push(stringStacks[i]);
  curStack++;
  if (curStack == 10) {
    endCheck = true;
    curStack = 1;
  } else endCheck = false;
}

let stacks2 = structuredClone(stacks);

function completeInstruction(instruction: string) {
  let amount = Number(instruction.split(" ")[1]);
  let from = Number(instruction.split(" ")[3]);
  let to = Number(instruction.split(" ")[5]);
  for (let i = amount; i > 0; i--) {
    stacks[to - 1].unshift(stacks[from - 1].shift());
  }
}

for (let instruction of instructions) {
  completeInstruction(instruction);
}

for (let stack of stacks) {
  answer += stack[0];
}

console.log("Answer to part one is " + answer);

function completeAltInstruction(instruction: string) {
  let amount = Number(instruction.split(" ")[1]);
  let from = Number(instruction.split(" ")[3]);
  let to = Number(instruction.split(" ")[5]);
  let buff: string[] = [];
  for (let i = amount; i >= 1; i--) {
    if (stacks2[from - 1].length <= 0) {
      break;
    }
    buff.push(stacks2[from - 1].shift());
  }
  buff.reverse();
  for (let val of buff) {
    stacks2[to - 1].unshift(val);
  }
}

for (let i = 0; i < instructions.length; i++) {
  completeAltInstruction(instructions[i]);
}

answer = "";

for (let stack of stacks2) {
  answer += stack[0];
}

console.log("Answer to part two is " + answer);

export {};
