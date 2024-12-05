package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D21_Part1(t *testing.T) {
	d := &D21{}
	res, err := d.part1(D21_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D21_PT1, res)
}

func Test_D21_Part2(t *testing.T) {
	d := &D21{}
	res, err := d.part2(D21_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D21_PT2, res)
}