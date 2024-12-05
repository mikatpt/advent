package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D07_Part1(t *testing.T) {
	d := &D07{}
	res, err := d.part1(D07_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D07_PT1, res)
}

func Test_D07_Part2(t *testing.T) {
	d := &D07{}
	res, err := d.part2(D07_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D07_PT2, res)
}