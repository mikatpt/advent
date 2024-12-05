package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D24_Part1(t *testing.T) {
	d := &D24{}
	res, err := d.part1(D24_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D24_PT1, res)
}

func Test_D24_Part2(t *testing.T) {
	d := &D24{}
	res, err := d.part2(D24_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D24_PT2, res)
}