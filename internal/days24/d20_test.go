package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D20_Part1(t *testing.T) {
	d := &D20{}
	res, err := d.part1(D20_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D20_PT1, res)
}

func Test_D20_Part2(t *testing.T) {
	d := &D20{}
	res, err := d.part2(D20_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D20_PT2, res)
}