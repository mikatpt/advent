package days24

import (
	"math"
	"slices"
	"strconv"
	"strings"
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D01 struct{}

const (
	D01_INPUT = `3   4
4   3
2   5
1   3
3   9
3   3`

	D01_INPUT_2 = D01_INPUT

	D01_PT1   = 11
	D01_PT2   = 31
	D01_PT1_R = 2176849
	D01_PT2_R = 23384288
)

func (d *D01) Solve() (int, int, error) {
	input, err := utils.GetInput(1)
	if err != nil {
		return 0, 0, err
	}

	p1, err := d.Part1(input)
	if err != nil {
		return p1, 0, err
	}
	p2, err := d.Part2(input)

	return p1, p2, err
}

func (d *D01) read(input string) ([]int, []int) {
	left, right := []int{}, []int{}
	for _, line := range strings.Split(input, "\n") {
		if len(line) == 0 {
			continue
		}
		l := strings.Split(line, "   ")
		first, second := l[0], l[1]
		f, _ := strconv.Atoi(first)
		s, _ := strconv.Atoi(second)
		left = append(left, f)
		right = append(right, s)
	}

	slices.Sort(left)
	slices.Sort(right)
	return left, right
}

func (d *D01) Part1(input string) (int, error) {
	left, right := d.read(input)

	var res int
	for i := 0; i < len(left); i++ {
		res += int(math.Abs(float64(left[i] - right[i])))
	}

	return res, nil
}

func (d *D01) Part2(input string) (int, error) {
	left, right := d.read(input)
	similarity := map[int]int{}
	for _, item := range right {
		similarity[item] += 1
	}

	var res int
	for i := 0; i < len(left); i++ {
		sim := left[i] * similarity[left[i]]
		res += sim
	}

	return res, nil
}

func Test_D01(t *testing.T) {
	utils.TestDay(
		t,
		&D01{},
		D01_INPUT,
		D01_INPUT_2,
		D01_PT1,
		D01_PT2,
		D01_PT1_R,
		D01_PT2_R,
		1,
	)
}
