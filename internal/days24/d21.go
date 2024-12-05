package days24

import (
	"github.com/mikatpt/advent/internal/utils"
)

type D21 struct{}

const (
	D21_INPUT = ``

	D21_PT1 = 0
	D21_PT2 = 0
)

func (d *D21) Solve() (int, int, error) {
	input, err := utils.GetInput(21)
	if err != nil {
		return 0, 0, err
	}

	p1, err := d.part1(input)
	if err != nil {
		return p1, 0, err
	}
	p2, err := d.part2(input)

	return p1, p2, err
}

func (d *D21) part1(input string) (int, error) {
	return 0, nil
}

func (d *D21) part2(input string) (int, error) {
	return 0, nil
}
