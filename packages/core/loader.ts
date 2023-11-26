/**
 * A class to handle batch processing of promises. Think dataloader outside graphql.
 *
 * # Example
 * ```ts
 * async function generateTypesForQueries(queries: string): Promise<void> {
 *     console.log(queries);
 * }
 *
 * const loader = new Loader(generateTypesForQueries);
 *
 * function generateTypeForQuery(query: string): void {
 *     loader.load(query);
 * }
 *
 * generateTypeForQuery("SELECT * FROM posts");
 * generateTypeForQuery("SELECT * FROM comments");
 * // ...
 *
 * // The following will be printed to console
 * // ["SELECT * FROM posts", "SELECT * FROM comments"]
 * ```
 */
export class Loader {
	queries: string[] = [];
	batchLoaderHasBeenCalled = false;
	batchLoader: (queries: string[]) => Promise<void>;

	constructor(batchLoader: (queries: string[]) => Promise<void>) {
		this.batchLoader = batchLoader;
	}

	async load(query: string) {
		return new Promise(async (resolve, reject) => {
			this.queries.push(query);
			process.nextTick(async () => {
				if (!this.batchLoaderHasBeenCalled) {
					this.batchLoader(this.queries);
					this.batchLoaderHasBeenCalled = true;
				}
				resolve(undefined);
			});
		});
	}
}
