package main

import (
	"fmt"
	"log"

	"github.com/pocketbase/dbx"
)

// TODO: improve implementation
func HandlerError(msg string, err error) {
	if err != nil {
		log.Println(msg)
		log.Fatalf("\n%v\n", err)
	}
}

func IsEmptyTable(db *dbx.DB, table string) bool {
	q := db.Select("count(1)").From(table)
	var count int
	err := q.Row(&count)
	HandlerError(fmt.Sprintf("[%s] error querying", table), err)

	empty := count <= 0
	if !empty {
		log.Printf("[%s] already seeded\n", table)
	}
	return empty
}

func InsertCollection[T any](db *dbx.DB, collection []*T, table string) {
	fmt.Println("")
	log.Printf("[%s] seeding\n", table)

	empty := IsEmptyTable(db, table)
	if !empty {
		if truncate {
			log.Printf("[%s] truncating\n", table)
			_, err := db.TruncateTable(table).Execute()
			HandlerError("error truncating", err)
		} else {
			return
		}
	}

	for _, item := range collection {
		err := db.Model(item).Insert()
		HandlerError("error inserting", err)
	}
}
