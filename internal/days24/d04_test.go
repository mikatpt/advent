package days24

import (
	"strings"
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D04 struct{}

const (
	D04_INPUT = `MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX`
	D04_INPUT_2 = D04_INPUT

	D04_PT1   = 18
	D04_PT1_R = 2521
	D04_PT2   = 9
	D04_PT2_R = 1912
)

func (d *D04) Part1(input string) (int, error) {
	return d.read(input)
}

func (d *D04) read(input string) (int, error) {
	matrix := [][]string{}
	xs := [][]int{}
	for i, line := range strings.Split(input, "\n") {
		if line == "" {
			continue
		}
		row := make([]string, 0, len(line))
		for j, chr := range line {
			c := string(chr)
			row = append(row, c)
			if c == "X" {
				xs = append(xs, []int{i, j})
			}
		}
		matrix = append(matrix, row)
	}

	dirs := [][]int{
		[]int{0, 1},
		[]int{0, -1},
		[]int{1, 0},
		[]int{-1, 0},
		[]int{-1, -1},
		[]int{-1, 1},
		[]int{1, 1},
		[]int{1, -1},
	}
	count := 0
	for _, coords := range xs {
		x, y := coords[0], coords[1]
		for _, dir := range dirs {
			dx, dy := dir[0], dir[1]
			d.dfs(matrix, x, y, dx, dy, []string{"M", "A", "S"}, &count)
		}
	}
	return count, nil
}

func (d *D04) dfs(matrix [][]string, x, y, dx, dy int, cache []string, count *int) {
	if len(cache) == 0 {
		*count++
		return
	}
	m, n := len(matrix), len(matrix[0])
	if x+dx == m || y+dy == n || x+dx < 0 || y+dy < 0 {
		return
	}
	x2, y2 := x+dx, y+dy
	if matrix[x2][y2] == cache[0] {
		d.dfs(matrix, x2, y2, dx, dy, cache[1:], count)
	}
}

func (d *D04) Part2(input string) (int, error) {
	matrix := [][]string{}
	xs := [][]int{}
	for i, line := range strings.Split(input, "\n") {
		if line == "" {
			continue
		}
		row := make([]string, 0, len(line))
		for j, chr := range line {
			c := string(chr)
			row = append(row, c)
			if c == "A" {
				xs = append(xs, []int{i, j})
			}
		}
		matrix = append(matrix, row)
	}

	count := 0
	for _, coords := range xs {
		x, y := coords[0], coords[1]
		leftDiag := d.checkOpposite(matrix, x+1, y+1, x-1, y-1)
		rightDiag := d.checkOpposite(matrix, x-1, y+1, x+1, y-1)
		if leftDiag && rightDiag {
			count++
		}
	}

	return count, nil
}

func (d *D04) invalid(matrix [][]string, x, y int) bool {
	return x == len(matrix) || y == len(matrix[0]) || x < 0 || y < 0
}

func (d *D04) checkOpposite(matrix [][]string, x, y, x2, y2 int) bool {
	valid := map[string]bool{"M": true, "S": true}
	if d.invalid(matrix, x, y) || d.invalid(matrix, x2, y2) {
		return false
	}
	if valid[matrix[x][y]] {
		delete(valid, matrix[x][y])
		return valid[matrix[x2][y2]]
	}
	return false
}

func Test_D04(t *testing.T) {
	utils.TestDay(
		t,
		&D04{},
		D04_INPUT,
		D04_INPUT_2,
		D04_PT1,
		D04_PT2,
		D04_PT1_R,
		D04_PT2_R,
		4,
	)
}
