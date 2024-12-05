package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D30_Part1(t *testing.T) {
	d := &D30{}
	res, err := d.part1(D30_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D30_PT1, res)
}

func Test_D30_Part2(t *testing.T) {
	d := &D30{}
	res, err := d.part2(D30_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D30_PT2, res)
}