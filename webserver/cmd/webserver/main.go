package main

import (
	"fmt"
	"io"
	"net/http"
)

func route(w http.ResponseWriter, r *http.Request) {

	fmt.Printf("got / request\n")
	io.WriteString(w, "Hello, world!")
}

func handler() {
	http.HandleFunc("/", route)
	err := http.ListenAndServe(":8080", nil)
	if err != nil {
		panic(err)
	}

}
func main() {
	handler()

}
