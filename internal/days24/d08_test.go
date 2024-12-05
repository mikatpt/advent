package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D08_Part1(t *testing.T) {
	d := &D08{}
	res, err := d.part1(D08_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D08_PT1, res)
}

func Test_D08_Part2(t *testing.T) {
	d := &D08{}
	res, err := d.part2(D08_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D08_PT2, res)
}