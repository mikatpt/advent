package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D21 struct{}

const (
	D21_INPUT   = ``
	D21_INPUT_2 = D21_INPUT

	D21_PT1   = 0
	D21_PT1_R = 0
	D21_PT2   = 0
	D21_PT2_R = 0
)

func (d *D21) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D21) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D21(t *testing.T) {
	utils.TestDay(
		t,
		&D21{},
		D21_INPUT,
		D21_INPUT_2,
		D21_PT1,
		D21_PT2,
		D21_PT1_R,
		D21_PT2_R,
		21,
	)
}
