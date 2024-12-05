package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D29_Part1(t *testing.T) {
	d := &D29{}
	res, err := d.part1(D29_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D29_PT1, res)
}

func Test_D29_Part2(t *testing.T) {
	d := &D29{}
	res, err := d.part2(D29_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D29_PT2, res)
}