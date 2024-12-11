package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D15 struct{}

const (
	D15_INPUT   = ``
	D15_INPUT_2 = D15_INPUT

	D15_PT1   = 0
	D15_PT1_R = 0
	D15_PT2   = 0
	D15_PT2_R = 0
)

func (d *D15) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D15) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D15(t *testing.T) {
	utils.TestDay(
		t,
		&D15{},
		D15_INPUT,
		D15_INPUT_2,
		D15_PT1,
		D15_PT2,
		D15_PT1_R,
		D15_PT2_R,
		15,
	)
}
