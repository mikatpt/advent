package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D14_Part1(t *testing.T) {
	d := &D14{}
	res, err := d.part1(D14_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D14_PT1, res)
}

func Test_D14_Part2(t *testing.T) {
	d := &D14{}
	res, err := d.part2(D14_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D14_PT2, res)
}