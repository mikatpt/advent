package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D20 struct{}

const (
	D20_INPUT   = ``
	D20_INPUT_2 = D20_INPUT

	D20_PT1   = 0
	D20_PT1_R = 0
	D20_PT2   = 0
	D20_PT2_R = 0
)

func (d *D20) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D20) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D20(t *testing.T) {
	utils.TestDay(
		t,
		&D20{},
		D20_INPUT,
		D20_INPUT_2,
		D20_PT1,
		D20_PT2,
		D20_PT1_R,
		D20_PT2_R,
		20,
	)
}
