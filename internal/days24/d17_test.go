package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D17 struct{}

const (
	D17_INPUT   = ``
	D17_INPUT_2 = D17_INPUT

	D17_PT1   = 0
	D17_PT1_R = 0
	D17_PT2   = 0
	D17_PT2_R = 0
)

func (d *D17) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D17) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D17(t *testing.T) {
	utils.TestDay(
		t,
		&D17{},
		D17_INPUT,
		D17_INPUT_2,
		D17_PT1,
		D17_PT2,
		D17_PT1_R,
		D17_PT2_R,
		17,
	)
}
