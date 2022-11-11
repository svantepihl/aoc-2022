import * as fs from 'fs';

const inputDataLines = fs.readFileSync('input.txt', 'utf-8');

const part = process.env.part || 'part1';

if (part === 'part1') console.log(getSolutionPartOne(inputDataLines));
else console.log(getSolutionPartTwo(inputDataLines));

export function getSolutionPartOne(inputDataLines: string): any {
  throw new Error('Function not implemented.');
}

export function getSolutionPartTwo(inputDataLines: string): any {
  throw new Error('Function not implemented.');
}
