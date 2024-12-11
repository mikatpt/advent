package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D31 struct{}

const (
	D31_INPUT   = ``
	D31_INPUT_2 = D31_INPUT

	D31_PT1   = 0
	D31_PT1_R = 0
	D31_PT2   = 0
	D31_PT2_R = 0
)

func (d *D31) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D31) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D31(t *testing.T) {
	utils.TestDay(
		t,
		&D31{},
		D31_INPUT,
		D31_INPUT_2,
		D31_PT1,
		D31_PT2,
		D31_PT1_R,
		D31_PT2_R,
		31,
	)
}
