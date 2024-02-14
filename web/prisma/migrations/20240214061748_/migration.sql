/*
  Warnings:

  - Added the required column `winner` to the `Bet` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Bet" (
    "id" BIGINT NOT NULL PRIMARY KEY,
    "eventId" BIGINT NOT NULL,
    "creatorId" TEXT NOT NULL,
    "winner" TEXT NOT NULL,
    "amount" BIGINT NOT NULL,
    "status" TEXT NOT NULL DEFAULT 'placing',
    CONSTRAINT "Bet_eventId_fkey" FOREIGN KEY ("eventId") REFERENCES "Event" ("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "Bet_creatorId_fkey" FOREIGN KEY ("creatorId") REFERENCES "User" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO "new_Bet" ("amount", "creatorId", "eventId", "id", "status") SELECT "amount", "creatorId", "eventId", "id", "status" FROM "Bet";
DROP TABLE "Bet";
ALTER TABLE "new_Bet" RENAME TO "Bet";
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
