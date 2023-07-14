const fs = require("fs");

let input: string = __dirname + "/input.txt";
let data: string = fs.readFileSync(input, "utf8");

class File {
  constructor(name: string, size: number) {
    this.name = name;
    this.size = size;
  }
  name: string;
  size: number;
}

class Directory {
  constructor(name: string) {
    this.name = name;
  }
  name: string;
  directories?: Directory[] = [];
  files?: File[] = [];

  ls() {
    for (let dir of this.directories!) {
      console.log(dir.name);
    }
    for (let file of this.files!) {
      console.log(file.name);
    }
  }

  cd(arg: string) {}

  calcSize() {
    let total: number = 0;
    for (let file of this.files!) {
      total += file.size;
    }
    if (this.directories!.length == 0) {
      return total;
    } else {
      for (let dir of this.directories!) {
        total += dir.calcSize()!;
      }
    }
  }
}

let root: any;

console.log(data.split("\r\n$ "));

function partOne(data: string[]) {
  let currentDir: string;
  root = new Directory("/");
  for (let line of data) {
    if (line.startsWith("cd")) {
      if (line.startsWith("cd")) {
      } else if (line.startsWith("ls")) {
      } else if (line.startsWith("dir")) {
        root.directories.push(new Directory(line.split(" ")[1]));
      } else {
        root.files.push(
          new File(line.split(" ")[0], Number(line.split(" ")[1]))
        );
      }
    }
  }
}

partOne(data.split("\r\n$ "));
