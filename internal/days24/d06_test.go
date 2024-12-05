package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D06_Part1(t *testing.T) {
	d := &D06{}
	res, err := d.part1(D06_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D06_PT1, res)
}

func Test_D06_Part2(t *testing.T) {
	d := &D06{}
	res, err := d.part2(D06_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D06_PT2, res)
}