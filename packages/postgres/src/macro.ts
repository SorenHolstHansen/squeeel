import { Loader } from "@squeal/core";
import { generateTypesForQueries } from "./generation/generateTypes";

const loader = new Loader(generateTypesForQueries);

export function q<T extends string>(query: T): T {
  loader.load(query);
  return query;
}
