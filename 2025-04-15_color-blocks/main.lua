local lick = require "lick"

function love.draw()
    local cell_size = 32

    local block_size = 30
    local block_offset = (cell_size - block_size) / 2

    for i = 1, 10, 1 do
        for j = 1, 16, 1 do
            love.graphics.rectangle(
                "fill",
                i * cell_size + block_offset, j * cell_size + block_offset,
                block_size, block_size
            )
        end
    end
end
