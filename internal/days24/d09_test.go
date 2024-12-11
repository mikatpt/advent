package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D09 struct{}

const (
	D09_INPUT   = ``
	D09_INPUT_2 = D09_INPUT

	D09_PT1   = 0
	D09_PT1_R = 0
	D09_PT2   = 0
	D09_PT2_R = 0
)

func (d *D09) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D09) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D09(t *testing.T) {
	utils.TestDay(
		t,
		&D09{},
		D09_INPUT,
		D09_INPUT_2,
		D09_PT1,
		D09_PT2,
		D09_PT1_R,
		D09_PT2_R,
		9,
	)
}
