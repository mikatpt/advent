package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D16 struct{}

const (
	D16_INPUT   = ``
	D16_INPUT_2 = D16_INPUT

	D16_PT1   = 0
	D16_PT1_R = 0
	D16_PT2   = 0
	D16_PT2_R = 0
)

func (d *D16) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D16) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D16(t *testing.T) {
	utils.TestDay(
		t,
		&D16{},
		D16_INPUT,
		D16_INPUT_2,
		D16_PT1,
		D16_PT2,
		D16_PT1_R,
		D16_PT2_R,
		16,
	)
}
