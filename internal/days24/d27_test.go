package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D27 struct{}

const (
	D27_INPUT   = ``
	D27_INPUT_2 = D27_INPUT

	D27_PT1   = 0
	D27_PT1_R = 0
	D27_PT2   = 0
	D27_PT2_R = 0
)

func (d *D27) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D27) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D27(t *testing.T) {
	utils.TestDay(
		t,
		&D27{},
		D27_INPUT,
		D27_INPUT_2,
		D27_PT1,
		D27_PT2,
		D27_PT1_R,
		D27_PT2_R,
		27,
	)
}
