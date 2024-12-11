package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D28 struct{}

const (
	D28_INPUT   = ``
	D28_INPUT_2 = D28_INPUT

	D28_PT1   = 0
	D28_PT1_R = 0
	D28_PT2   = 0
	D28_PT2_R = 0
)

func (d *D28) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D28) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D28(t *testing.T) {
	utils.TestDay(
		t,
		&D28{},
		D28_INPUT,
		D28_INPUT_2,
		D28_PT1,
		D28_PT2,
		D28_PT1_R,
		D28_PT2_R,
		28,
	)
}
