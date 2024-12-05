package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D09_Part1(t *testing.T) {
	d := &D09{}
	res, err := d.part1(D09_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D09_PT1, res)
}

func Test_D09_Part2(t *testing.T) {
	d := &D09{}
	res, err := d.part2(D09_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D09_PT2, res)
}