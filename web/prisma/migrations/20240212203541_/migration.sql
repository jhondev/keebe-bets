/*
  Warnings:

  - Added the required column `teamA` to the `Event` table without a default value. This is not possible if the table is not empty.
  - Added the required column `teamB` to the `Event` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Event" (
    "id" BIGINT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "gameSlug" TEXT NOT NULL,
    "status" TEXT NOT NULL,
    "teamA" TEXT NOT NULL,
    "teamB" TEXT NOT NULL,
    "pointsA" INTEGER,
    "pointsB" INTEGER,
    "winner" TEXT,
    "createdAt" DATETIME NOT NULL
);
INSERT INTO "new_Event" ("createdAt", "gameSlug", "id", "name", "status", "winner") SELECT "createdAt", "gameSlug", "id", "name", "status", "winner" FROM "Event";
DROP TABLE "Event";
ALTER TABLE "new_Event" RENAME TO "Event";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
