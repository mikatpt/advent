package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D19 struct{}

const (
	D19_INPUT   = ``
	D19_INPUT_2 = D19_INPUT

	D19_PT1   = 0
	D19_PT1_R = 0
	D19_PT2   = 0
	D19_PT2_R = 0
)

func (d *D19) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D19) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D19(t *testing.T) {
	utils.TestDay(
		t,
		&D19{},
		D19_INPUT,
		D19_INPUT_2,
		D19_PT1,
		D19_PT2,
		D19_PT1_R,
		D19_PT2_R,
		19,
	)
}
