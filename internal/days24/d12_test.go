package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D12 struct{}

const (
	D12_INPUT   = ``
	D12_INPUT_2 = D12_INPUT

	D12_PT1   = 0
	D12_PT1_R = 0
	D12_PT2   = 0
	D12_PT2_R = 0
)

func (d *D12) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D12) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D12(t *testing.T) {
	utils.TestDay(
		t,
		&D12{},
		D12_INPUT,
		D12_INPUT_2,
		D12_PT1,
		D12_PT2,
		D12_PT1_R,
		D12_PT2_R,
		12,
	)
}
