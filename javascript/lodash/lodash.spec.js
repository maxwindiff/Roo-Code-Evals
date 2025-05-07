const _ = require("./lodash");

describe("lodash", () => {
  test("map should transform each element in an array", () => {
    const array = [1, 2, 3, 4];
    const result = _.map(array, (n) => n * 2);
    expect(result).toEqual([2, 4, 6, 8]);
  });

  test("filter should return elements that pass the predicate", () => {
    const array = [1, 2, 3, 4, 5, 6];
    const result = _.filter(array, (n) => n % 2 === 0);
    expect(result).toEqual([2, 4, 6]);
  });

  test("reduce should accumulate values", () => {
    const array = [1, 2, 3, 4];
    const result = _.reduce(array, (sum, n) => sum + n, 0);
    expect(result).toEqual(10);
  });
});
