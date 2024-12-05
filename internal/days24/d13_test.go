package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D13_Part1(t *testing.T) {
	d := &D13{}
	res, err := d.part1(D13_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D13_PT1, res)
}

func Test_D13_Part2(t *testing.T) {
	d := &D13{}
	res, err := d.part2(D13_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D13_PT2, res)
}