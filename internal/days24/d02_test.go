package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D02_Part1(t *testing.T) {
	d := &D02{}
	res, err := d.part1(D02_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D02_PT1, res)
}

func Test_D02_Part2(t *testing.T) {
	d := &D02{}
	res, err := d.part2(D02_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D02_PT2, res)
}