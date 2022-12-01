import { getSolutionPartOne, getSolutionPartTwo } from ".";

const exampleInput = `1000
2000
3000

4000

5000
6000

7000
8000
9000

10000`;

describe("getSolutionPartOne", () => {
  it("should return the correct solution", () => {
    expect(getSolutionPartOne(exampleInput)).toBe("24000");
  });
});

describe("getSolutionPartTwo", () => {
  it("should return the correct solution", () => {
    expect(getSolutionPartTwo(exampleInput)).toBe("45000");
  });
});
