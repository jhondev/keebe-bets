// import { drizzle } from 'drizzle-orm/better-sqlite3';
// import Database from 'better-sqlite3';
// import { config } from '../../../drizzle.config';
// export const sqlite = new Database(config.dbCredentials.url);
// export const db = drizzle(sqlite);

import { PrismaClient } from '@prisma/client';
export const db = new PrismaClient();
