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
			// Ideally we should use process.nextTick, but I think the various postgres clients are clogging up the event loop, which means it would never run. Have not found a way to debug bun's event loop to see what's up here. Seems that this is a combination of macros, nextTick, and postgres libs.
			// Anyways, setTimeout might end up being better, as for huge repos, a custom timeout might have to be set by the user.
			setTimeout(async () => {
				if (!this.batchLoaderHasBeenCalled) {
					this.batchLoader(this.queries);
					this.batchLoaderHasBeenCalled = true;
				}
				resolve(undefined);
			});
		});
	}
}
