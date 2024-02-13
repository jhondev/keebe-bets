-- CreateTable
CREATE TABLE "Event" (
    "id" BIGINT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "gameSlug" TEXT NOT NULL,
    "winner" TEXT NOT NULL,
    "createdAt" DATETIME NOT NULL
);

-- CreateTable
CREATE TABLE "Bet" (
    "id" BIGINT NOT NULL PRIMARY KEY,
    "eventId" BIGINT NOT NULL,
    "creatorId" TEXT NOT NULL,
    "amount" BIGINT NOT NULL,
    CONSTRAINT "Bet_eventId_fkey" FOREIGN KEY ("eventId") REFERENCES "Event" ("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "Bet_creatorId_fkey" FOREIGN KEY ("creatorId") REFERENCES "User" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "BetParticipants" (
    "betId" BIGINT NOT NULL,
    "participantId" TEXT NOT NULL,

    PRIMARY KEY ("betId", "participantId"),
    CONSTRAINT "BetParticipants_betId_fkey" FOREIGN KEY ("betId") REFERENCES "Bet" ("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "BetParticipants_participantId_fkey" FOREIGN KEY ("participantId") REFERENCES "User" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "User" (
    "id" TEXT NOT NULL PRIMARY KEY
);
