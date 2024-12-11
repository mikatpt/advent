package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D14 struct{}

const (
	D14_INPUT   = ``
	D14_INPUT_2 = D14_INPUT

	D14_PT1   = 0
	D14_PT1_R = 0
	D14_PT2   = 0
	D14_PT2_R = 0
)

func (d *D14) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D14) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D14(t *testing.T) {
	utils.TestDay(
		t,
		&D14{},
		D14_INPUT,
		D14_INPUT_2,
		D14_PT1,
		D14_PT2,
		D14_PT1_R,
		D14_PT2_R,
		14,
	)
}
