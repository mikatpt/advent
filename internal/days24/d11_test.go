package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D11_Part1(t *testing.T) {
	d := &D11{}
	res, err := d.part1(D11_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D11_PT1, res)
}

func Test_D11_Part2(t *testing.T) {
	d := &D11{}
	res, err := d.part2(D11_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D11_PT2, res)
}