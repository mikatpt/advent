package utils

import (
	"fmt"
	"os"
	"text/template"
)

const (
	TEMPLATE = `package days24

import (
	"github.com/mikatpt/advent/internal/utils"
)

type D{{ .dayPadded }} struct{}

const (
	D{{ .dayPadded }}_INPUT = ` + "`" + `3   4
4   3
2   5
1   3
3   9
3   3` + "`" + `

	D{{ .dayPadded }}_PT1 = 0
	D{{ .dayPadded }}_PT2 = 0
)

func (d *D{{ .dayPadded }}) Solve() (int, int, error) {
	input, err := utils.GetInput({{ .day }})
	if err != nil {
		return 0, 0, err
	}

	p1, err := d.part1(input)
	if err != nil {
		return p1, 0, err
	}
	p2, err := d.part2(input)

	return p1, p2, err
}

func (d *D{{ .dayPadded }}) part1(input string) (int, error) {
	return 0, nil
}

func (d *D{{ .dayPadded }}) part2(input string) (int, error) {
	return 0, nil
}`

	TEST_TEMPLATE = `package days24

import (
	"testing"

	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

func Test_D{{ .dayPadded }}_Part1(t *testing.T) {
	d := &D{{ .dayPadded }}{}
	res, err := d.part1(D{{ .dayPadded }}_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D{{ .dayPadded }}_PT1, res)
}

func Test_D{{ .dayPadded }}_Part2(t *testing.T) {
	d := &D{{ .dayPadded }}{}
	res, err := d.part2(D{{ .dayPadded }}_INPUT)
	if err != nil {
		t.Fatal(aurora.Red("failed to execute"), err)
	}

	assert.Equal(t, D{{ .dayPadded }}_PT2, res)
}`
)

func GenYear(yearSuffix int) error {
	if err := os.MkdirAll(fmt.Sprintf("internal/days%d", yearSuffix), os.ModePerm); err != nil {
		return err
	}

	templates := []string{TEMPLATE, TEST_TEMPLATE}
	for i := 1; i <= 31; i++ {
		for _, tmp := range templates {
			if err := genDay(tmp, yearSuffix, i); err != nil {
				return err
			}
		}
	}
	return nil
}

func genDay(tmp string, yearSuffix, day int) error {
	isTest := tmp == TEST_TEMPLATE

	tmpl, err := template.New("day").Parse(tmp)
	if err != nil {
		return err
	}
	data := map[string]interface{}{
		"day":       day,
		"dayPadded": fmt.Sprintf("%02d", day),
	}
	path := fmt.Sprintf("internal/days%d/d%02d", yearSuffix, day)
	if isTest {
		path += "_test"
	}
	path += ".go"
	if _, err := os.Stat(path); err == nil {
		return nil
	}

	file, err := os.Create(path)
	if err != nil {
		return err
	}
	defer file.Close()

	if err := tmpl.Execute(file, data); err != nil {
		return err
	}

	return nil
}
