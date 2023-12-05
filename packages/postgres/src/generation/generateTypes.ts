import { ExportKeyword, JsonValueType } from "@squeeel/core";
import path from "path";
import { Client } from "pg";
import { Project, printNode } from "ts-morph";
import { ts } from "ts-morph";
import { GeneratedTypes, generateTypesForQuery } from "./generateTypesForQuery";
import { generateTypesForEnums } from "./enums";
const { factory } = ts;

export async function generateTypesForQueries(
  queries: string[],
): Promise<void> {
  const database_url = "postgres://postgres:postgres@localhost:5432/postgres";
  const client = new Client({ connectionString: database_url });
  await client.connect();
  const enums = await generateTypesForEnums(client);
  const types = await Promise.all(
    queries.map(async (query) => {
      return [query, await generateTypesForQuery(query, enums, client)] as [
        string,
        GeneratedTypes,
      ];
    }),
  );
  await client.end();

  const project = new Project();
  const sourceFile = project.createSourceFile("_.ts");
  const GeneratedTypes = factory.createTypeAliasDeclaration(
    [ExportKeyword],
    factory.createIdentifier("GeneratedTypes"),
    undefined,
    factory.createTypeLiteralNode(
      types.map(([query, { outputType, argType }]) =>
        factory.createPropertySignature(
          undefined,
          factory.createStringLiteral(query),
          undefined,
          factory.createTypeLiteralNode([
            factory.createPropertySignature(
              undefined,
              factory.createIdentifier("inputType"),
              undefined,
              argType,
            ),
            factory.createPropertySignature(
              undefined,
              factory.createIdentifier("outputType"),
              undefined,
              outputType,
            ),
          ]),
        ),
      ),
    ),
  );
  sourceFile.addStatements(printNode(JsonValueType));
  sourceFile.addStatements(printNode(GeneratedTypes));

  Bun.write(
    path.join(__dirname, "../../_squeeel_generated_types.ts"),
    sourceFile.getFullText(),
  );
}
