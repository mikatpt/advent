package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D11 struct{}

const (
	D11_INPUT   = ``
	D11_INPUT_2 = D11_INPUT

	D11_PT1   = 0
	D11_PT1_R = 0
	D11_PT2   = 0
	D11_PT2_R = 0
)

func (d *D11) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D11) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D11(t *testing.T) {
	utils.TestDay(
		t,
		&D11{},
		D11_INPUT,
		D11_INPUT_2,
		D11_PT1,
		D11_PT2,
		D11_PT1_R,
		D11_PT2_R,
		11,
	)
}
