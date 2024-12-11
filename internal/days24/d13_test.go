package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D13 struct{}

const (
	D13_INPUT   = ``
	D13_INPUT_2 = D13_INPUT

	D13_PT1   = 0
	D13_PT1_R = 0
	D13_PT2   = 0
	D13_PT2_R = 0
)

func (d *D13) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D13) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D13(t *testing.T) {
	utils.TestDay(
		t,
		&D13{},
		D13_INPUT,
		D13_INPUT_2,
		D13_PT1,
		D13_PT2,
		D13_PT1_R,
		D13_PT2_R,
		13,
	)
}
