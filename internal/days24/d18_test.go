package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D18 struct{}

const (
	D18_INPUT   = ``
	D18_INPUT_2 = D18_INPUT

	D18_PT1   = 0
	D18_PT1_R = 0
	D18_PT2   = 0
	D18_PT2_R = 0
)

func (d *D18) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D18) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D18(t *testing.T) {
	utils.TestDay(
		t,
		&D18{},
		D18_INPUT,
		D18_INPUT_2,
		D18_PT1,
		D18_PT2,
		D18_PT1_R,
		D18_PT2_R,
		18,
	)
}
