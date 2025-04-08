package parser

import (
	"strconv"
	"github.com/gen2brain/raylib-go/raylib"
)

const STEP = 50

func Window(points [][2]float32) {

	var WWITDH int32 = 1200
	var WHEIGHT int32 = 800

	rl.SetConfigFlags(rl.FlagMsaa4xHint | rl.FlagWindowResizable | rl.FlagVsyncHint)
	rl.InitWindow(WWITDH, WHEIGHT, "xPlotGo")
	
	GeistNerdFont := rl.LoadFontEx("assets/GeistMonoNerdFont-SemiBold.otf", 128, nil, 0)

	camera := rl.Camera2D{
		// 0, 0 is now the world middle
		Offset: rl.NewVector2(float32(WWITDH), float32(WHEIGHT)),
		Target: rl.NewVector2(0, 0),
		Rotation: 0.0,
		Zoom: 1.0,
	}

	for !rl.WindowShouldClose(){

		delta := rl.GetFrameTime()
		//update camera Offset for Windows rezising
		camera.Offset = rl.NewVector2(float32(rl.GetScreenWidth()/2), float32(rl.GetScreenHeight()/2))

		//zoom
		if rl.IsKeyDown(rl.KeyUp) {
			camera.Zoom += 1.3 * delta
		}else if rl.IsKeyDown(rl.KeyDown) {
			camera.Zoom -= 1.3 * delta
		}	
		if camera.Zoom > 3.0 {
			camera.Zoom = 3.0 
		} else if camera.Zoom < 0.4 {
			camera.Zoom = 0.4
		}

		//Shifting
		if rl.IsKeyDown(rl.KeyW) {
			camera.Target.Y -= 600 * delta
		}
		if rl.IsKeyDown(rl.KeyS) {
			camera.Target.Y += 600 * delta
		}
		if rl.IsKeyDown(rl.KeyA) {
			camera.Target.X -= 600 * delta
		}
		if rl.IsKeyDown(rl.KeyD) {
			camera.Target.X += 600 * delta
		}
		if camera.Target.X > float32(WWITDH/2) {
			camera.Target.X = float32(WWITDH/2)
		}
		if camera.Target.X < float32(-WWITDH/2) {
			camera.Target.X = float32(-WWITDH/2)
		}
		if camera.Target.Y > float32(WHEIGHT/2) {
			camera.Target.Y = float32(WHEIGHT/2)
		}
		if camera.Target.Y < float32(-WHEIGHT/2) {
			camera.Target.Y = float32(-WHEIGHT/2)
		}



		//reset
		if rl.IsKeyDown(rl.KeyR) {
			camera.Target = rl.NewVector2(0, 0)
			camera.Zoom = 1
		}

		rl.BeginDrawing()
		rl.ClearBackground(rl.Black)
		
		rl.BeginMode2D(camera)	

		Axis(&camera)
		Graph(&camera, points)
		
		rl.EndMode2D()

		rl.DrawTextEx(GeistNerdFont, "'W A S D' to shift", rl.NewVector2(50, 25), 21, 1, rl.White)
		rl.DrawTextEx(GeistNerdFont, "'R' to reset", rl.NewVector2(50, 50), 21, 1, rl.White)
		rl.DrawTextEx(GeistNerdFont, "Arrow-keys to zoom", rl.NewVector2(50, 75), 21, 1, rl.White)
		rl.DrawTextEx(GeistNerdFont, "Zoom: " + strconv.FormatFloat(float64(camera.Zoom), 'f', 1, 32), rl.NewVector2(50, 100), 21, 1, rl.White)

		rl.EndDrawing()
	}
}

func Axis(camera *rl.Camera2D) {
	// draws exactly to the screen sides (+10) so it doesn't flicker 
	rl.DrawLine(-int32(camera.Offset.X/camera.Zoom) + int32(camera.Target.X) - 10, 0, int32(camera.Offset.X/camera.Zoom) + int32(camera.Target.X) + 10, 0, rl.White)
	rl.DrawLine(0, -int32(camera.Offset.Y/camera.Zoom) + int32(camera.Target.Y) - 10, 0, int32(camera.Offset.Y/camera.Zoom) + int32(camera.Target.Y) + 10, rl.White)

	for i := 0.0; i <= float64(camera.Offset.X / (3 / camera.Zoom)); i = i+0.4 {
		if i != 0 {
			// X-Axis
			rl.DrawLine(int32(i*STEP), -7, int32(i*STEP), 7, rl.White)
			rl.DrawLine(int32(-i*STEP), -7, int32(-i*STEP), 7, rl.White)
			// Y-Axis
			rl.DrawLine(-7, int32(i*STEP), 7, int32(i*STEP), rl.White)
			rl.DrawLine(-7, int32(-i*STEP), 7, int32(-i*STEP), rl.White)
			
		}
	}
}

func Graph(camera *rl.Camera2D, points [][2]float32) {

	var pointsV []rl.Vector2
	
	//convert points into Vector2 from raylib
	for _, p := range points {
		pointsV = append(pointsV, rl.NewVector2(p[0] * STEP, -p[1] * STEP))
	}

	rl.DrawSplineCatmullRom(pointsV, 2, rl.Blue)
	
}
