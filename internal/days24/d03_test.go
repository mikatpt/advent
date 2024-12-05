package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D03_Part1(t *testing.T) {
	d := &D03{}
	res, err := d.part1(D03_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D03_PT1, res)
}

func Test_D03_Part2(t *testing.T) {
	d := &D03{}
	res, err := d.part2(D03_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D03_PT2, res)
}