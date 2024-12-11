package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D24 struct{}

const (
	D24_INPUT   = ``
	D24_INPUT_2 = D24_INPUT

	D24_PT1   = 0
	D24_PT1_R = 0
	D24_PT2   = 0
	D24_PT2_R = 0
)

func (d *D24) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D24) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D24(t *testing.T) {
	utils.TestDay(
		t,
		&D24{},
		D24_INPUT,
		D24_INPUT_2,
		D24_PT1,
		D24_PT2,
		D24_PT1_R,
		D24_PT2_R,
		24,
	)
}
