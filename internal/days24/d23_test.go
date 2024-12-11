package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D23 struct{}

const (
	D23_INPUT   = ``
	D23_INPUT_2 = D23_INPUT

	D23_PT1   = 0
	D23_PT1_R = 0
	D23_PT2   = 0
	D23_PT2_R = 0
)

func (d *D23) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D23) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D23(t *testing.T) {
	utils.TestDay(
		t,
		&D23{},
		D23_INPUT,
		D23_INPUT_2,
		D23_PT1,
		D23_PT2,
		D23_PT1_R,
		D23_PT2_R,
		23,
	)
}
