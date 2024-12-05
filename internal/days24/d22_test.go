package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D22_Part1(t *testing.T) {
	d := &D22{}
	res, err := d.part1(D22_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D22_PT1, res)
}

func Test_D22_Part2(t *testing.T) {
	d := &D22{}
	res, err := d.part2(D22_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D22_PT2, res)
}