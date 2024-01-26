package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"time"

	"github.com/gertd/go-pluralize"
	"github.com/joho/godotenv"
	"github.com/pocketbase/dbx"

	_ "github.com/jackc/pgx/v5/stdlib"
)

var props = []byte("{}")
var creatorID = "9a997d13-897f-4c18-9cd6-e4cc28829d90"
var createdAt = time.Now().UTC()

var truncate bool

func main() {
	err := godotenv.Load()
	HandlerError("Error loading .env file", err)

	flag.BoolVar(&truncate, "truncate", false, "truncates tables before seeding")
	flag.Parse()

	fmt.Printf("\ntruncate: %v\n\n", truncate)

	log.Println("connecting to database")
	db, err := dbx.Open("pgx", os.Getenv("DB_URL"))
	HandlerError("connecting to database", err)

	// use pluralize for model names
	p := pluralize.NewClient()
	db.TableMapper = func(a interface{}) string {
		return p.Plural(dbx.GetTableName(a))
	}

	Seed(db)
}

func Seed(db *dbx.DB) {
	// Seed(db)

	fmt.Println("")
	log.Println("seeding successful")
}
