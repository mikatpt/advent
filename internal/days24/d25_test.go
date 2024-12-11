package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D25 struct{}

const (
	D25_INPUT   = ``
	D25_INPUT_2 = D25_INPUT

	D25_PT1   = 0
	D25_PT1_R = 0
	D25_PT2   = 0
	D25_PT2_R = 0
)

func (d *D25) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D25) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D25(t *testing.T) {
	utils.TestDay(
		t,
		&D25{},
		D25_INPUT,
		D25_INPUT_2,
		D25_PT1,
		D25_PT2,
		D25_PT1_R,
		D25_PT2_R,
		25,
	)
}
