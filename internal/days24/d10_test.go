package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D10 struct{}

const (
	D10_INPUT   = ``
	D10_INPUT_2 = D10_INPUT

	D10_PT1   = 0
	D10_PT1_R = 0
	D10_PT2   = 0
	D10_PT2_R = 0
)

func (d *D10) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D10) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D10(t *testing.T) {
	utils.TestDay(
		t,
		&D10{},
		D10_INPUT,
		D10_INPUT_2,
		D10_PT1,
		D10_PT2,
		D10_PT1_R,
		D10_PT2_R,
		10,
	)
}
