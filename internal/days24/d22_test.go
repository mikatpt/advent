package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D22 struct{}

const (
	D22_INPUT   = ``
	D22_INPUT_2 = D22_INPUT

	D22_PT1   = 0
	D22_PT1_R = 0
	D22_PT2   = 0
	D22_PT2_R = 0
)

func (d *D22) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D22) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D22(t *testing.T) {
	utils.TestDay(
		t,
		&D22{},
		D22_INPUT,
		D22_INPUT_2,
		D22_PT1,
		D22_PT2,
		D22_PT1_R,
		D22_PT2_R,
		22,
	)
}
