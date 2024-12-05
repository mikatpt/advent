package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D28_Part1(t *testing.T) {
	d := &D28{}
	res, err := d.part1(D28_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D28_PT1, res)
}

func Test_D28_Part2(t *testing.T) {
	d := &D28{}
	res, err := d.part2(D28_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D28_PT2, res)
}