import { MyType } from './_squeal_generated_types';

export async function perform<T extends string>(query: T) {
  return query as T extends keyof MyType ? MyType[T] : unknown;
}
