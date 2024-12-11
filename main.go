package main

import (
	"fmt"

	"github.com/joho/godotenv"
	"github.com/logrusorgru/aurora/v4"
	"github.com/mikatpt/advent/internal/utils"
)

func main() {
	_ = godotenv.Load()

	err := utils.GenYear(utils.YEAR)
	if err != nil {
		fmt.Println(aurora.Red("Err generating:"), err)
	}
}
