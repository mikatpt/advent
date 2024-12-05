package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D23_Part1(t *testing.T) {
	d := &D23{}
	res, err := d.part1(D23_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D23_PT1, res)
}

func Test_D23_Part2(t *testing.T) {
	d := &D23{}
	res, err := d.part2(D23_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D23_PT2, res)
}