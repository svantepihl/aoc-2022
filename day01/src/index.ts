import * as fs from "fs";

const inputString = fs.readFileSync("input.txt", "utf-8");

const parseInput = (input: string): number[] => {
  return input
    .split("\n\n")
    .map((group) => group.split("\n"))
    .map((elf) => elf.reduce((acc, cur) => acc + parseInt(cur, 10), 0))
    .sort((a, b) => b - a);
};

export function getSolutionPartOne(input: string): number {
  return parseInput(input)[0];
}

export function getSolutionPartTwo(input: string): number {
  return parseInput(input)
    .splice(0, 3)
    .reduce((acc, cur) => acc + cur, 0);
}

const part = process.env.part || "part1";

if (part === "part1") console.log(getSolutionPartOne(inputString));
else console.log(getSolutionPartTwo(inputString));
