package days24

import (
	"math"
	"strconv"
	"strings"
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D02 struct{}

const (
	D02_INPUT = `7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9`
	D02_INPUT_2 = D02_INPUT

	D02_PT1 = 2
	D02_PT2 = 4

	D02_PT1_R = 359
	D02_PT2_R = 418
)

func (d *D02) read(input string) [][]int {
	matrix := [][]int{}
	for _, line := range strings.Split(input, "\n") {
		row := []int{}
		if len(line) == 0 {
			continue
		}
		for _, nStr := range strings.Split(line, " ") {
			n, _ := strconv.Atoi(nStr)
			row = append(row, n)
		}
		matrix = append(matrix, row)
	}
	return matrix
}

func (d *D02) Part1(input string) (int, error) {
	matrix := d.read(input)
	// levels must be all increasing or all decreasing.
	// adjacent levels must differ by at least one, at most three.
	safeReports := 0

	for _, row := range matrix {
		if d.rowIsSafe(row, -1) {
			safeReports++
		}
	}

	return safeReports, nil
}

func (d *D02) rowIsSafe(row []int, skippedIdx int) bool {
	isSafe := true

	var isDecreasing bool
	i, j := 0, 1
	switch skippedIdx {
	case 0:
		i, j = 1, 2
	case 1:
		j = 2
	}
	for {
		if i == len(row)-1 || j == len(row) {
			break
		}

		first := (skippedIdx == 0 && i == 1)
		if !(first || i == 0) && isDecreasing != (row[i] > row[j]) {
			isSafe = false
			break
		}
		isDecreasing = row[i] > row[j]

		if !d.isSafe(row[i], row[j]) {
			isSafe = false
			break
		}
		i++
		j++
		if i == skippedIdx {
			i++
		}
		if j == skippedIdx {
			j++
		}
	}
	return isSafe
}
func (d *D02) isSafe(a, b int) bool {
	diff := int(math.Abs(float64(a - b)))
	goodDiff := diff >= 1 && diff <= 3

	return goodDiff
}

func (d *D02) Part2(input string) (int, error) {
	matrix := d.read(input)

	safeReports := 0
	for _, row := range matrix {
		for i := -1; i < len(row); i++ {
			if d.rowIsSafe(row, i) {
				safeReports++
				break
			}
		}
	}

	return safeReports, nil
}

func Test_D02(t *testing.T) {
	utils.TestDay(
		t,
		&D02{},
		D02_INPUT,
		D02_INPUT_2,
		D02_PT1,
		D02_PT2,
		D02_PT1_R,
		D02_PT2_R,
		2,
	)
}
