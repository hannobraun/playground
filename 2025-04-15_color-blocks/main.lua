local lick = require "lick"

function love.draw()
    local field_size_x = 10
    local field_size_y = 16

    local cell_size = 32

    local block_size = 30
    local block_offset = (cell_size - block_size) / 2

    for i = 1, field_size_x do
        for j = 1, field_size_y do
            love.graphics.rectangle(
                "fill",
                i * cell_size + block_offset, j * cell_size + block_offset,
                block_size, block_size
            )
        end
    end
end
