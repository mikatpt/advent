package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D16_Part1(t *testing.T) {
	d := &D16{}
	res, err := d.part1(D16_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D16_PT1, res)
}

func Test_D16_Part2(t *testing.T) {
	d := &D16{}
	res, err := d.part2(D16_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D16_PT2, res)
}