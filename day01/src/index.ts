import * as fs from 'fs';

const inputString = fs.readFileSync('input.txt', 'utf-8');

const part = process.env.part || 'part1';

if (part === 'part1') console.log(getSolutionPartOne(inputString));
else console.log(getSolutionPartTwo(inputString));

export function getSolutionPartOne(inputString: string): string {
  throw new Error('Function not implemented.');
}

export function getSolutionPartTwo(inputString: string): string {
  throw new Error('Function not implemented.');
}
