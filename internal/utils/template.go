package utils

import (
	"fmt"
	"os"
	"text/template"
)

const TEMPLATE = `package days24

import (
	"testing"

	"github.com/mikatpt/advent/internal/utils"
)

type D{{ .dayPadded }} struct{}

const (
	D{{ .dayPadded }}_INPUT   = ` + "`" + "`" + `
	D{{ .dayPadded }}_INPUT_2 = D{{ .dayPadded }}_INPUT

	D{{ .dayPadded }}_PT1   = 0
	D{{ .dayPadded }}_PT1_R = 0
	D{{ .dayPadded }}_PT2   = 0
	D{{ .dayPadded }}_PT2_R = 0
)

func (d *D{{ .dayPadded }}) Part1(input string) (int, error) {
	return 0, nil
}

func (d *D{{ .dayPadded }}) Part2(input string) (int, error) {
	return 0, nil
}

func Test_D{{ .dayPadded }}(t *testing.T) {
	utils.TestDay(
		t,
		&D{{ .dayPadded }}{},
		D{{ .dayPadded }}_INPUT,
		D{{ .dayPadded }}_INPUT_2,
		D{{ .dayPadded }}_PT1,
		D{{ .dayPadded }}_PT2,
		D{{ .dayPadded }}_PT1_R,
		D{{ .dayPadded }}_PT2_R,
		{{ .day }},
	)
}
`

func GenYear(yearSuffix int) error {
	if err := os.MkdirAll(fmt.Sprintf("internal/days%d", yearSuffix), os.ModePerm); err != nil {
		return err
	}

	for i := 1; i <= 31; i++ {
		if err := genDay(TEMPLATE, yearSuffix, i); err != nil {
			return err
		}
	}
	return nil
}

func genDay(tmp string, yearSuffix, day int) error {
	tmpl, err := template.New("day").Parse(tmp)
	if err != nil {
		return err
	}
	data := map[string]interface{}{
		"day":       day,
		"dayPadded": fmt.Sprintf("%02d", day),
	}
	path := fmt.Sprintf("internal/days%d/d%02d_test.go", yearSuffix, day)
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
