import { Loader } from "./index";
import { expect, test, mock } from "bun:test";

test.todo("Loader batches queries", () => {
  const batchFunction = mock(async (queries) => queries);
  const loader = new Loader(batchFunction);

  loader.load("A");
  loader.load("B");
  loader.load("C");

  expect(batchFunction).toHaveBeenCalledWith(["A", "B", "C"]);
});
