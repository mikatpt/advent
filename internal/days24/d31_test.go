package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D31_Part1(t *testing.T) {
	d := &D31{}
	res, err := d.part1(D31_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D31_PT1, res)
}

func Test_D31_Part2(t *testing.T) {
	d := &D31{}
	res, err := d.part2(D31_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D31_PT2, res)
}