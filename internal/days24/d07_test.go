package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D07 struct{}

const (
	D07_INPUT   = ``
	D07_INPUT_2 = D07_INPUT

	D07_PT1   = 0
	D07_PT1_R = 0
	D07_PT2   = 0
	D07_PT2_R = 0
)

func (d *D07) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D07) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D07(t *testing.T) {
	utils.TestDay(
		t,
		&D07{},
		D07_INPUT,
		D07_INPUT_2,
		D07_PT1,
		D07_PT2,
		D07_PT1_R,
		D07_PT2_R,
		7,
	)
}
