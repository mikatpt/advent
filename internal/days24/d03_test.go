package days24

import (
	"strconv"
	"strings"
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D03 struct{ valid map[rune]bool }

const (
	D03_INPUT   = `xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))`
	D03_INPUT_2 = `xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))`

	D03_PT1   = 161
	D03_PT2   = 0
	D03_PT1_R = 188741603
	D03_PT2_R = 0
)

func (d *D03) Part1(input string) (int, error) {
	d.valid = map[rune]bool{
		'0': true,
		'1': true,
		'2': true,
		'3': true,
		'4': true,
		'5': true,
		'6': true,
		'7': true,
		'8': true,
		'9': true,
		',': true,
	}
	res := 0
	str := input
outer:
	for {
		i := strings.Index(str, "mul(")
		if i == -1 {
			break
		}

		var j int
		s := str
		for {
			e := strings.Index(s, ")")
			if j == -1 {
				break outer
			}

			j += e
			if j > i {
				break
			}
			j += 1
			s = s[e+1:]
		}

		valid, a, b, operation := d.checkValid(str[i : j+1])
		if !valid {
			str = str[i+4:]
			continue
		}
		res += d.operate(a, b, operation, true)
		str = str[j+1:]
	}
	return res, nil
}

func (d *D03) checkValid(str string) (bool, string, string, string) {
	if len(str) < 8 {
		return false, "", "", ""
	}
	inner := str[4 : len(str)-1]
	for _, chr := range inner {
		if !d.valid[chr] {
			return false, "", "", ""
		}
	}
	ops := strings.Split(inner, ",")
	if len(ops) != 2 {
		return false, "", "", ""
	}
	a, b := ops[0], ops[1]

	return true, a, b, str[:3]
}

func (d *D03) operate(aStr, bStr, op string, exec bool) int {
	if !exec {
		return 0
	}
	a, _ := strconv.Atoi(aStr)
	b, _ := strconv.Atoi(bStr)
	switch op {
	case "mul":
		return a * b
	case "div":
		return a / b
	case "add":
		return a + b
	case "min":
		return a - b
	default:
		panic("unimplemented " + op)
	}
}

func (d *D03) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D03(t *testing.T) {
	utils.TestDay(
		t,
		&D03{},
		D03_INPUT,
		D03_INPUT_2,
		D03_PT1,
		D03_PT2,
		D03_PT1_R,
		D03_PT2_R,
		3,
	)
}
