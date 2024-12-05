package main

import (
	"fmt"
	"os"

	"github.com/joho/godotenv"
	"github.com/logrusorgru/aurora/v4"
	d "github.com/mikatpt/advent/internal/days24"
	"github.com/mikatpt/advent/internal/utils"
)

var days = map[string]utils.Day{
	"1":  &d.D01{},
	"2":  &d.D02{},
	"3":  &d.D03{},
	"4":  &d.D04{},
	"5":  &d.D05{},
	"6":  &d.D06{},
	"7":  &d.D07{},
	"8":  &d.D08{},
	"9":  &d.D09{},
	"10": &d.D10{},
	"11": &d.D11{},
	"12": &d.D12{},
	"13": &d.D13{},
	"14": &d.D14{},
	"15": &d.D15{},
	"16": &d.D16{},
	"17": &d.D17{},
	"18": &d.D18{},
	"19": &d.D19{},
	"20": &d.D20{},
	"21": &d.D21{},
	"22": &d.D22{},
	"23": &d.D23{},
	"24": &d.D24{},
	"25": &d.D25{},
	"26": &d.D26{},
	"27": &d.D27{},
	"28": &d.D28{},
	"29": &d.D29{},
	"30": &d.D30{},
	"31": &d.D31{},
}

func main() {
	_ = godotenv.Load()

	args := os.Args
	if len(args) < 2 {
		fmt.Println(aurora.Red("Please specify a day!"))
		os.Exit(2)
	}
	err := utils.GenYear(utils.YEAR)
	if err != nil {
		fmt.Println(aurora.Red("Err generating:"), err)
	}

	day := args[1]
	p1, p2, err := days[day].Solve()
	if err != nil {
		fmt.Println(aurora.Red("Err executing:"), err)
		os.Exit(2)
	}
	fmt.Println(aurora.Green("p1:"), p1)
	fmt.Println(aurora.Green("p2:"), p2)
}
