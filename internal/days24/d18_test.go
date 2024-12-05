package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D18_Part1(t *testing.T) {
	d := &D18{}
	res, err := d.part1(D18_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D18_PT1, res)
}

func Test_D18_Part2(t *testing.T) {
	d := &D18{}
	res, err := d.part2(D18_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D18_PT2, res)
}