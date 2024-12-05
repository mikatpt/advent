package days24

import (
	"github.com/mikatpt/advent/internal/utils"
)

type D13 struct{}

const (
	D13_INPUT = ``

	D13_PT1 = 0
	D13_PT2 = 0
)

func (d *D13) Solve() (int, int, error) {
	input, err := utils.GetInput(13)
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

func (d *D13) part1(input string) (int, error) {
	return 0, nil
}

func (d *D13) part2(input string) (int, error) {
	return 0, nil
}
