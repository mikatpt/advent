package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D10_Part1(t *testing.T) {
	d := &D10{}
	res, err := d.part1(D10_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D10_PT1, res)
}

func Test_D10_Part2(t *testing.T) {
	d := &D10{}
	res, err := d.part2(D10_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D10_PT2, res)
}