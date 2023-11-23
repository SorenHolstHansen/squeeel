import { q } from '@squeal/postgres' with { type: 'macro' };
import { perform } from '@squeal/postgres';

const res = await perform(q("SELECT * FROM post"));
const res2 = await perform(q("SELECT id FROM post"));
