/*
  Warnings:

  - Added the required column `status` to the `Event` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Event" (
    "id" BIGINT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "gameSlug" TEXT NOT NULL,
    "status" TEXT NOT NULL,
    "winner" TEXT,
    "createdAt" DATETIME NOT NULL
);
INSERT INTO "new_Event" ("createdAt", "gameSlug", "id", "name", "winner") SELECT "createdAt", "gameSlug", "id", "name", "winner" FROM "Event";
DROP TABLE "Event";
ALTER TABLE "new_Event" RENAME TO "Event";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
