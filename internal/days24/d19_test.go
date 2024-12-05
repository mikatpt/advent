package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D19_Part1(t *testing.T) {
	d := &D19{}
	res, err := d.part1(D19_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D19_PT1, res)
}

func Test_D19_Part2(t *testing.T) {
	d := &D19{}
	res, err := d.part2(D19_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D19_PT2, res)
}