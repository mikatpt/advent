package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D29 struct{}

const (
	D29_INPUT   = ``
	D29_INPUT_2 = D29_INPUT

	D29_PT1   = 0
	D29_PT1_R = 0
	D29_PT2   = 0
	D29_PT2_R = 0
)

func (d *D29) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D29) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D29(t *testing.T) {
	utils.TestDay(
		t,
		&D29{},
		D29_INPUT,
		D29_INPUT_2,
		D29_PT1,
		D29_PT2,
		D29_PT1_R,
		D29_PT2_R,
		29,
	)
}
