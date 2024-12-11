package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D08 struct{}

const (
	D08_INPUT   = ``
	D08_INPUT_2 = D08_INPUT

	D08_PT1   = 0
	D08_PT1_R = 0
	D08_PT2   = 0
	D08_PT2_R = 0
)

func (d *D08) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D08) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D08(t *testing.T) {
	utils.TestDay(
		t,
		&D08{},
		D08_INPUT,
		D08_INPUT_2,
		D08_PT1,
		D08_PT2,
		D08_PT1_R,
		D08_PT2_R,
		8,
	)
}
