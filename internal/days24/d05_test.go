package days24

import (
	"strconv"
	"strings"
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D05 struct {
	nodes map[string][]string
}

// type D5Node struct{
// 	name string
// 	deps []*D5Node
// }

const (
	D05_INPUT = `47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47`
	D05_INPUT_2 = D05_INPUT

	D05_PT1   = 143
	D05_PT1_R = 0
	D05_PT2   = 0
	D05_PT2_R = 0
)

/*
build up a graph where for each rule x|y,
where each node y has dependency x
== x before y
== y after x

i.e. directed edge from x -> y

check each update in order. if node has dependencies on future items, FAIL.

75,47,61,53,29

save all in FUT. pop off.
75: check deps NOT in FUT: 97, VALID.
47: check deps NOT in FUT: 75, 97. VALID.
61: check deps NOT in FUT: 97,47,75. VALID.
53: check deps NOT in FUT: 75,61,97. VALID.
29: check deps NOT in FUT: 75,97,53,47. VALID.


*/

func (d *D05) Part1(input string) (int, error) {
	sum := 0
	d.nodes = map[string][]string{}
	for _, line := range strings.Split(input, "\n") {
		if strings.Contains(line, "|") {
			s := strings.Split(line, "|")
			first, second := s[0], s[1]
			d.nodes[second] = append(d.nodes[second], first)
		} else if strings.Contains(line, ",") {
			sum += d.process(line)
		}
	}
	return sum, nil
}

func (d *D05) process(line string) int {
	fut := []string{}
	futMap := map[string]bool{}
	ls := strings.Split(line, ",")
	for _, n := range ls {
		fut = append(fut, n)
		futMap[n] = true
	}
	for {
		if len(fut) == 0 {
			break
		}
		n := fut[0]
		deps := d.nodes[n]
		fut = fut[1:]
		futMap[n] = false
		for _, dep := range deps {
			if futMap[dep] {
				return 0
			}
		}
	}
	midpoint := len(ls) / 2
	int, _ := strconv.Atoi(ls[midpoint])
	return int
}

func (d *D05) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D05(t *testing.T) {
	utils.TestDay(
		t,
		&D05{},
		D05_INPUT,
		D05_INPUT_2,
		D05_PT1,
		D05_PT2,
		D05_PT1_R,
		D05_PT2_R,
		5,
	)
}
