package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D17_Part1(t *testing.T) {
	d := &D17{}
	res, err := d.part1(D17_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D17_PT1, res)
}

func Test_D17_Part2(t *testing.T) {
	d := &D17{}
	res, err := d.part2(D17_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D17_PT2, res)
}