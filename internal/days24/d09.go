package days24

import (
	"github.com/mikatpt/advent/internal/utils"
)

type D09 struct{}

const (
	D09_INPUT = ``

	D09_PT1 = 0
	D09_PT2 = 0
)

func (d *D09) Solve() (int, int, error) {
	input, err := utils.GetInput(9)
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

func (d *D09) part1(input string) (int, error) {
	return 0, nil
}

func (d *D09) part2(input string) (int, error) {
	return 0, nil
}
