package parser

import (
	"strconv"
	"github.com/gen2brain/raylib-go/raylib"
)


func Window(points [][2]float32) {

	var WWITDH int32 = 1200
	var WHEIGHT int32 = 800
	
	rl.InitWindow(WWITDH, WHEIGHT, "xPlotGo")
	rl.SetTargetFPS(30)

	camera := rl.Camera2D{
		// 0, 0 is now the world middle
		Offset: rl.NewVector2(float32(WWITDH), float32(WHEIGHT)),
		Target: rl.NewVector2(0, 0),
		Rotation: 0.0,
		Zoom: 1.0,
	}

	for !rl.WindowShouldClose(){

		//update camera Offset for Windows rezising
		camera.Offset = rl.NewVector2(float32(rl.GetScreenWidth()/2), float32(rl.GetScreenHeight()/2))

		camera.Zoom += float32(rl.GetMouseWheelMove()) * 0.1
		if camera.Zoom > 3.0 {
			camera.Zoom = 3.0
		} else if camera.Zoom < 0.4 {
			camera.Zoom = 0.4
		}

		rl.BeginDrawing()
		rl.ClearBackground(rl.Black)
		rl.BeginMode2D(camera)	

		Axis(&camera)
		Graph(&camera, points)
		
		rl.EndMode2D()

		rl.DrawText("Mouse Wheel to zoom", 50, 50, 16, rl.White)
		rl.DrawText("Zoom: " + strconv.FormatFloat(float64(camera.Zoom), 'f', 1, 32), 50, 100, 16, rl.White)
		
		rl.EndDrawing()
	}
}

func Axis(camera *rl.Camera2D) {
	// draws exactly to the screen sides (+10) so it doesn't flicker 
	rl.DrawLine(-int32(camera.Offset.X/camera.Zoom) - 10, 0, int32(camera.Offset.X/camera.Zoom) + 10, 0, rl.White)
	rl.DrawLine(0, -int32(camera.Offset.Y/camera.Zoom) - 10, 0, int32(camera.Offset.Y/camera.Zoom) + 10, rl.White)

	for i := 0.0; i <= float64(camera.Offset.X / (5 / camera.Zoom)); i = i+0.4 {
		if i != 0 {
			// X-Axis
			rl.DrawLine(int32(i*50), -7, int32(i*50), 7, rl.White)
			rl.DrawLine(int32(-i*50), -7, int32(-i*50), 7, rl.White)
			// Y-Axis
			rl.DrawLine(-7, int32(i*50), 7, int32(i*50), rl.White)
			rl.DrawLine(-7, int32(-i*50), 7, int32(-i*50), rl.White)
			
		}
	}
}

func Graph(camera *rl.Camera2D, points [][2]float32) {

}
