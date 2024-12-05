package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D01_Part1(t *testing.T) {
	d := &D01{}
	res, err := d.part1(D01_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D01_PT1, res)
}

func Test_D01_Part2(t *testing.T) {
	d := &D01{}
	res, err := d.part2(D01_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D01_PT2, res)
}
