package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D25_Part1(t *testing.T) {
	d := &D25{}
	res, err := d.part1(D25_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D25_PT1, res)
}

func Test_D25_Part2(t *testing.T) {
	d := &D25{}
	res, err := d.part2(D25_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D25_PT2, res)
}