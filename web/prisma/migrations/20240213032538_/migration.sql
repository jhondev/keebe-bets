/*
  Warnings:

  - Made the column `pointsA` on table `Event` required. This step will fail if there are existing NULL values in that column.
  - Made the column `pointsB` on table `Event` required. This step will fail if there are existing NULL values in that column.

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
    "pointsA" TEXT NOT NULL,
    "pointsB" TEXT NOT NULL,
    "winner" TEXT,
    "startAt" DATETIME,
    "createdAt" DATETIME NOT NULL
);
INSERT INTO "new_Event" ("createdAt", "gameSlug", "id", "name", "pointsA", "pointsB", "startAt", "status", "teamA", "teamB", "winner") SELECT "createdAt", "gameSlug", "id", "name", "pointsA", "pointsB", "startAt", "status", "teamA", "teamB", "winner" FROM "Event";
DROP TABLE "Event";
ALTER TABLE "new_Event" RENAME TO "Event";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
