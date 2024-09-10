package main
func calculateNextState(p golParams, world [][]byte) [][]byte {
    newWorld := make([][]byte, p.imageHeight)
    for i := range newWorld {
        newWorld[i] = make([]byte, p.imageWidth)
    }

    // go through each row and column and calculate if the cell should be alive or dead
    for i := 0; i < p.imageHeight; i++ {
        for j := 0; j < p.imageWidth; j++ {
            aliveNeighbors := 0
            // check the 8 neighbors of the cell
            for x := -1; x <= 1; x++ {
                for y := -1; y <= 1; y++ {
                    if x == 0 && y == 0 {
                        continue
                    }
                    // check if the neighbor is alive
                    x_neighbour := (i + x + p.imageHeight) % p.imageHeight
                    y_neighbour := (j + y + p.imageHeight) % p.imageHeight

                    if world[x_neighbour][y_neighbour] == 255 {
                        aliveNeighbors++
                    }
                }
            }
            if world[i][j] == 255 {
                if aliveNeighbors < 2 || aliveNeighbors > 3 {
                    newWorld[i][j] = 0
                } else {
                    newWorld[i][j] = 255
                }
            } else {
                if aliveNeighbors == 3 {
                    newWorld[i][j] = 255
                } else {
                    newWorld[i][j] = 0
                }
            }
        }
    }
    return newWorld
}

func calculateAliveCells(p golParams, world [][]byte) []cell {
    var aliveCells []cell
    for i := 0; i < p.imageHeight; i++ {
        for j := 0; j < p.imageWidth; j++ {
            if world[i][j] == 255 {
                aliveCells = append(aliveCells, cell{i, j})
            }
        }
    }
    return aliveCells
}