package main
func calculateNextState(p golParams, world [][]byte) [][]byte {
    newWorld := make([][]byte, p.imageWidth)
    for i := range newWorld {
        newWorld[i] = make([]byte, p.imageHeight)
    }

    // go through each row and column and calculate if the cell should be alive or dead
    for i := 0; i < p.imageWidth; i++ {
        for j := 0; j < p.imageHeight; j++ {
            aliveNeighbors := 0
            // check the 8 neighbors of the cell
            for x := -1; x <= 1; x++ {
                for y := -1; y <= 1; y++ {
                    if x == 0 && y == 0 {
                        continue
                    }
                    // check if the neighbor is alive
                    x_neighbour := (i + x + p.imageWidth) % p.imageWidth
                    y_neighbour := (j + y + p.imageHeight) % p.imageHeight

                    if world[y_neighbour][x_neighbour] == 255 {
                        aliveNeighbors++
                    }
                }
            }
            if world[j][i] == 255 {
                if aliveNeighbors < 2 || aliveNeighbors > 3 {
                    newWorld[j][i] = 0
                } else {
                    newWorld[j][i] = 255
                }
            } else {
                if aliveNeighbors == 3 {
                    newWorld[j][i] = 255
                } else {
                    newWorld[j][i] = 0
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
            if world[j][i] == 255 {
                aliveCells = append(aliveCells, cell{i,j})
            }
        }
    }
    return aliveCells
}