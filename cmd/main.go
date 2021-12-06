package main

import (
	"fmt"
	"io/ioutil"
	"os"
)

func main() {
	input := getInput()

	output := d1_2(input)

	fmt.Println(output)
}

func getInput() string {
	f := os.Args[1]

	i, err := ioutil.ReadFile(f)
	if err != nil {
		panic("Couldn't read file!")
	}

	return string(i)
}
