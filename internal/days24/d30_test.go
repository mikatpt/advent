package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D30 struct{}

const (
	D30_INPUT   = ``
	D30_INPUT_2 = D30_INPUT

	D30_PT1   = 0
	D30_PT1_R = 0
	D30_PT2   = 0
	D30_PT2_R = 0
)

func (d *D30) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D30) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D30(t *testing.T) {
	utils.TestDay(
		t,
		&D30{},
		D30_INPUT,
		D30_INPUT_2,
		D30_PT1,
		D30_PT2,
		D30_PT1_R,
		D30_PT2_R,
		30,
	)
}
