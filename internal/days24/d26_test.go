package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D26 struct{}

const (
	D26_INPUT   = ``
	D26_INPUT_2 = D26_INPUT

	D26_PT1   = 0
	D26_PT1_R = 0
	D26_PT2   = 0
	D26_PT2_R = 0
)

func (d *D26) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D26) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D26(t *testing.T) {
	utils.TestDay(
		t,
		&D26{},
		D26_INPUT,
		D26_INPUT_2,
		D26_PT1,
		D26_PT2,
		D26_PT1_R,
		D26_PT2_R,
		26,
	)
}
