package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D15_Part1(t *testing.T) {
	d := &D15{}
	res, err := d.part1(D15_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D15_PT1, res)
}

func Test_D15_Part2(t *testing.T) {
	d := &D15{}
	res, err := d.part2(D15_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D15_PT2, res)
}