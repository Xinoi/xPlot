package main

import (
	"xPlotGo/parser"
	"fmt"
)

func main() {

	raw := parser.GetRawInput()
	raw_list := parser.Split(raw)
	fmt.Println(raw_list)

	validated, err := parser.Validate(raw_list)
	fmt.Println(validated)
	
	rpn := parser.Shunting_Yard(validated)
	fmt.Printf("reverse polish notation: %s\n", rpn)

	var tt parser.TokenTree
	err = tt.IntoTree(rpn)
	if err != nil {
		fmt.Printf("Error getting Tree: %s", err.Error())
	}
	fmt.Println(tt)

	calculated, err := tt.CalculatePoints(20)
	if err != nil {
		println("Error calculating: %s", err.Error())
	}
	fmt.Println(calculated)

	parser.Window(calculated)
	
}
