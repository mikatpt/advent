package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D04_Part1(t *testing.T) {
	d := &D04{}
	res, err := d.part1(D04_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D04_PT1, res)
}

func Test_D04_Part2(t *testing.T) {
	d := &D04{}
	res, err := d.part2(D04_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D04_PT2, res)
}