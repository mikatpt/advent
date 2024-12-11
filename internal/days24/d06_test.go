package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D06 struct{}

const (
	D06_INPUT   = ``
	D06_INPUT_2 = D06_INPUT

	D06_PT1   = 0
	D06_PT1_R = 0
	D06_PT2   = 0
	D06_PT2_R = 0
)

func (d *D06) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D06) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D06(t *testing.T) {
	utils.TestDay(
		t,
		&D06{},
		D06_INPUT,
		D06_INPUT_2,
		D06_PT1,
		D06_PT2,
		D06_PT1_R,
		D06_PT2_R,
		6,
	)
}
