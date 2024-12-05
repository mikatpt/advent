package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D05_Part1(t *testing.T) {
	d := &D05{}
	res, err := d.part1(D05_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D05_PT1, res)
}

func Test_D05_Part2(t *testing.T) {
	d := &D05{}
	res, err := d.part2(D05_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D05_PT2, res)
}